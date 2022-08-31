package com.mrtan.rust_jni

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.mrtanrust_jni.ui.theme.Rust_jniTheme
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        nativeInit()
        setContent {
            Rust_jniTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Column {
                        Greeting(helloJni())
                        Button(onClick = { nativeCall("Call Native Method") }) {
                            Text(text = "Call Native Method")
                        }
                    }
                }
            }
        }
    }

    external fun helloJni():String;
    external fun nativeInit();
    external fun nativeCall(msg: String);

    fun callBack(msg: String){
        runOnUiThread {
            Toast.makeText(this, msg,Toast.LENGTH_SHORT).show()
        }
    }
    companion object{
        init {
            System.loadLibrary("rust")
        }
    }
}

@Composable
fun Greeting(name: String) {
    Text(text = "Hello $name!")
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    Rust_jniTheme {
        Greeting("Android")
    }
}