use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    // Handle Android-specific linking for ORT only if building for Android OS
    if target.contains("android") && target_os == "android" {
        println!("cargo:rerun-if-changed=android-app/app/src/main/jniLibs");
        
        // Link to the built ORT library for Android
        if target.contains("aarch64") {
            println!("cargo:rustc-link-search=native=android-app/app/src/main/jniLibs/arm64-v8a");
        } else if target.contains("armv7") {
            println!("cargo:rustc-link-search=native=android-app/app/src/main/jniLibs/armeabi-v7a");
        } else if target.contains("i686") {
            println!("cargo:rustc-link-search=native=android-app/app/src/main/jniLibs/x86");
        } else if target.contains("x86_64") {
            println!("cargo:rustc-link-search=native=android-app/app/src/main/jniLibs/x86_64");
        }
        
        // Link to ORT library (will be built from source)
        println!("cargo:rustc-link-lib=dylib=onnxruntime");
        
        // Link to required Android system libraries
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=c++");
    }
    
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=ANDROID_SDK_HOME");
}
