
use wasmer::{imports, Function, Instance, Module, Store, TypedFunction, Value};

fn main() -> anyhow::Result<()> {

    let mut store = Store::default();

    let module = Module::from_file(
        &store,
        "../wasm-mod/target/wasm32-unknown-unknown/debug/wasm_mod.wasm",
    )?;

    let imports = {
        imports! {
            "aaa" => {
                "bbb" => Function::new_typed(&mut store, bbb)
            }
        }
    };

    let instance = Instance::new(&mut store, &module, &imports)?;

    let add_one = instance.exports.get_function("add_one")?;

    let result = add_one.call(&mut store, &[Value::I32(12)])?;

    println!("result: {}", result[0]);

    Ok(())
}

fn bbb(a: i32) -> i32 {
    a + 2
}
