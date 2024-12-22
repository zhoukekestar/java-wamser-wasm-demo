// wasm 的 import 函数
#[link(wasm_import_module = "aaa")]
extern {
    fn bbb(idx: i32) -> i32;
}

// wasm 的导出函数
#[no_mangle]
pub extern fn add_one(x: i32) -> i32 {
    let a;
    unsafe {
        a = bbb(x);
    }
    a + 1
}