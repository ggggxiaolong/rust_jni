plugins {
    id ("com.android.application")
    id ("org.jetbrains.kotlin.android")
    id ("org.mozilla.rust-android-gradle.rust-android")
}

android {
    compileSdk  = 32
    ndkVersion = "23.1.7779620"
    defaultConfig {
        applicationId ="com.mrtan.rust_jni"
        minSdk =24
        targetSdk =32
        versionCode =1
        versionName ="1.0"

        testInstrumentationRunner ="androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary =true
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles( getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
    compileOptions {
        sourceCompatibility =JavaVersion.VERSION_1_8
        targetCompatibility =JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        compose =true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = "1.2.0-beta02"
    }
    packagingOptions {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

cargo {
    module = "../rust"
    targets = listOf("arm64")
    libname = "librust"
    libname = "rust"
    profile = "release"
    targetIncludes = arrayOf("librust.so")
}
//tasks.register<Copy>("copyArm") {
//    from("./src/rust/target/armv7-linux-androideabi/release/")
//    into("./src/main/jniLibs/armeabi-v7a/")
//    include("*.so")
//}

tasks.register<Copy>("copyArm64") {
    from("../rust/target/aarch64-linux-android/release/")
    into("./src/main/jniLibs/arm64-v8a/")
    include("*.so")
}

tasks.register("copySo") {
//    dependsOn("copyArm")
    dependsOn("copyArm64")
}

tasks.register("genSo") {
    dependsOn("cargoBuild")
    dependsOn("copySo")
    dependsOn("clean")
}

dependencies {
    implementation("androidx.core:core-ktx:1.8.0")
    implementation("androidx.compose.ui:ui:1.2.0-beta02")
    implementation("androidx.compose.material3:material3:1.0.0-alpha12")
    implementation("androidx.compose.ui:ui-tooling-preview:1.2.0-beta02")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.4.0")
    implementation("androidx.activity:activity-compose:1.4.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.3")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    androidTestImplementation("androidx.compose.ui:ui-test-junit4:1.2.0-beta02")
    debugImplementation("androidx.compose.ui:ui-tooling:1.2.0-beta02")
    debugImplementation("androidx.compose.ui:ui-test-manifest:1.2.0-beta02")
}