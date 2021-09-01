extern crate jni;

use std::ffi::c_void;

use jni::{JavaVM, JNIEnv, NativeMethod};
use jni::objects::{JClass, JString};
use jni::strings::JNIString;
use jni::sys::{jint, JNI_ERR, jstring};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_shenghaiyang_rap_MainActivity_stringFromJNI(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    let out = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    out.into_inner()
}

#[no_mangle]
pub fn dynamic_register_method(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    let out = env
        .new_string(format!("Hello, dynamic {}!", input))
        .expect("Couldn't create java string!");

    out.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn JNI_OnLoad(vm: JavaVM, _reserved: *mut c_void) -> jint {
    let env = vm.get_env().unwrap();
    let version = env.get_version().unwrap().into();

    let class = env.find_class("com/shenghaiyang/rap/MainActivity").unwrap();

    let methods = [
        NativeMethod {
            name: JNIString::from("dynamicRegisterMethod"),
            sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
            fn_ptr: dynamic_register_method as *mut c_void,
        }
    ];

    let result = env.register_native_methods(class, &methods);
    if result.is_ok() {
        version
    } else {
        JNI_ERR
    }
}
