#![cfg(target_os = "android")]
#![allow(non_snake_case)]

//////////////////////////////////////////////////
// Using

pub mod runner;

use jni_sys::JNIEnv;
use jni::objects::{JClass, JString};
use jni_sys::{jstring, jboolean, jobject, jint, JNI_TRUE};
use ndk::native_window::NativeWindow;


//////////////////////////////////////////////////
// Entry point for android

// #[ndk_glue::main(backtrace)]
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on", logger(level = "info", tag = "fatal")))]
fn main() {
    runner::start();
}

#[no_mangle]
pub unsafe extern fn Java_com_example_rust_1demo_RustUtils_drawColor__Landroid_view_Surface_2I(env: *mut JNIEnv, _: JClass, surface: jobject, color: jint) -> jboolean {
    println!("call Java_com_example_rust_1demo_RustUtils_drawColor__Landroid_view_Surface_2I"); 
    ndk_glue::set_native_window(NativeWindow::from_surface(env, surface));
    runner::start();
    0
}