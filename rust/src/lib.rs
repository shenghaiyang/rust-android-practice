extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

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
