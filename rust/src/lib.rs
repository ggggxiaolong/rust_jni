extern crate jni;

use std::ffi::{CString};
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use jni::{JNIEnv, NativeMethod,JavaVM};
use jni::objects::{GlobalRef, JObject, JString, JValue, JClass};
use jni::sys::{jint, JNI_ERR};
use lazy_static::lazy_static;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// pub type Callback = unsafe extern "C" fn(*const c_char) -> ();
lazy_static! {
    static ref JVM_GLOBAL: Mutex<Option<JavaVM>> = Mutex::default();
    static ref SENDER: Mutex<Option<Sender<String>>> = Mutex::default();
    static ref JNI_CALLBACK: Mutex<Option<GlobalRef>> = Mutex::default();
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut c_void) -> jint {
    tracing_subscriber::registry().with(tracing_android::layer("com.mrtan.rust_jni").unwrap()).init();
    let methods = &[
        NativeMethod {
            name: "helloJni".into(),
            sig: "()Ljava/lang/String;".into(),
            fn_ptr: native_hello as *mut c_void,
        },
        NativeMethod {
            name: "nativeInit".into(),
            sig: "()V".into(),
            fn_ptr: native_init as *mut c_void,
        },
        NativeMethod {
            name: "nativeCall".into(),
            sig: "(Ljava/lang/String;)V".into(),
            fn_ptr: native_callback as *mut c_void,
        },
    ];
    let class_name = "com/mrtan/rust_jni/MainActivity";
    let version: jint = register_natives(&jvm, class_name, methods);
    let mut jvm_ref = JVM_GLOBAL.lock().unwrap();
    *jvm_ref = Some(jvm);
    let (tx, rx) = channel(24);
    let mut sender = SENDER.lock().unwrap();
    *sender = Some(tx);
    run(rx);
    version
}

unsafe fn register_natives(jvm: &JavaVM, class_name: &str, methods: &[NativeMethod]) -> jint {
    let env: JNIEnv = jvm.get_env().unwrap();
    let jni_version = env.get_version().unwrap();
    let version: jint = jni_version.into();

    tracing::info!("JNI Version : {:#?} ", jni_version);

    let clazz = match env.find_class(class_name) {
        Ok(clazz) => clazz,
        Err(e) => {
            tracing::error!("java class not found : {:?}", e);
            return JNI_ERR;
        }
    };
    let result = env.register_native_methods(clazz, &methods);

    if result.is_ok() {
        tracing::info!("register_natives : succeed");
        version
    } else {
        tracing::error!("register_natives : failed ");
        JNI_ERR
    }
}

#[no_mangle]
pub fn native_hello<'a>(env: JNIEnv<'a>, _obj: JObject) -> JString<'a> {
    env.new_string("native_hello from jni").unwrap()
}

#[no_mangle]
pub fn native_init(env: JNIEnv, obj: JObject) {
    let callback = env.new_global_ref(obj).unwrap();
    let mut callback_ref = JNI_CALLBACK.lock().unwrap();
    *callback_ref = Some(callback);
}

#[no_mangle]
pub fn native_callback(env: JNIEnv, _obj: JObject, msg: JString) {
    let lock = SENDER.lock().unwrap();
    let sender = lock.as_ref().cloned();
    drop(lock);
    if let Some(sender) = sender{
        if let Ok(msg) = env.get_string(msg){
            tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async move{
                sender.send(msg.into()).await;
                tracing::info!("send ok");
            });
        }
    }
}

fn run (mut rcv: Receiver<String>){
    std::thread::spawn(move ||{
        let runtime = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
        runtime.block_on(async move {
            loop {
                tokio::select! {
                    Some(msg) = rcv.recv() => {
                        tracing::info!("receive msg {}", &msg);
                        send_msg(msg);
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(5)) =>{
                        tracing::info!("timeout");
                    }
                    else => {
                        tracing::info!("finished");
                    }
                }
            }
        })
    });
}

fn send_msg(msg: String){
    let jvm = JVM_GLOBAL.lock().unwrap();
    if (*jvm).is_none() {
        return;
    }
    let callback = JNI_CALLBACK.lock().unwrap();
    if (*callback).is_none() {
        return;
    }
    tracing::info!("start send msg: {}", &msg);
    let jvm: &JavaVM = (*jvm).as_ref().unwrap();
    if let Ok(env) = jvm.attach_current_thread_permanently() {
        let callback: JObject = (*callback).as_ref().unwrap().as_obj();
        let msg = JValue::Object(env.new_string(&msg).unwrap().into());
        match env.call_method(
            callback,
            "callBack",
            "(Ljava/lang/String;)V",
            &[msg],
        ){
            Ok(_r) => {
                tracing::info!("send succeed: {:?}", msg);
            }
            Err(e) => {
                tracing::error!("send failed : {:?}", e);
            }
        }
    }
}