use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_example_WASMContainer_hello<'local>(
    mut env: JNIEnv<'local>,

    class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let output = env.new_string("OK").expect("can't create string");
    output
}

#[no_mangle]
pub extern "system" fn Java_com_example_WASMContainer_handle<'local>(
    mut env: JNIEnv<'local>,

    class: JClass<'local>,
    request: JString<'local>,
    context: JObject<'local>,
) -> JString<'local> {

    let request_str = env.get_string(&request).unwrap();
    let request_string = request_str.to_str().unwrap();

    // 调用 java 函数
    let invoke_result: JString<'_> = env
        .call_method(
            context,
            "invoke",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JValue::Object(&env.new_string("from_wasm").unwrap())],
        )
        .unwrap()
        .l()
        .unwrap()
        .into();

    let invoke_result_str = env.get_string(&invoke_result).unwrap();
    let invoke_result_string = invoke_result_str.to_str().unwrap();

    let mut final_result = String::from(" final_result ");
    final_result.push_str(&request_string);
    final_result.push_str(&invoke_result_string);

    let output = env.new_string(final_result).expect("can't create string");
    output
}
