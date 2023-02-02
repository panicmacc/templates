// If the version of rust used is less than v1.63, please uncomment the follow attribute.
// #![feature(explicit_generic_args_with_impl_trait)]

use std::fs;
use wasmedge_sdk::{
    error::HostFuncError, host_function, params, wat2wasm, Caller, Executor, ImportObjectBuilder,
    Module, Store, WasmValue,
};

// We define a function to act as our "env" "say_hello" function imported in the
// Wasm program above.
#[host_function]
pub fn say_hello(caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");

    // get executor from caller
    let executor = caller.executor();
    assert!(executor.is_some());

    // get module instance from caller
    let instance = caller.instance();
    if let Some(instance) = instance {
        assert_eq!(instance.name(), Some("extern".to_string()));
        assert_eq!(instance.func_count(), 1);
        assert_eq!(instance.memory_count(), 0);
        assert_eq!(instance.global_count(), 0);
        assert_eq!(instance.table_count(), 0);
    }

    // get memory from caller
    let mem = caller.memory(0);
    assert!(mem.is_none());

    Ok(vec![])
}

// See here for example of calling a host function from a wasi app written in Rust:
// https://www.secondstate.io/articles/extend-webassembly/
// See also wasmedge's bindgen bits, which provide macros to handle the wasm<>rust parameter
// type conversions automatically.

#[cfg_attr(test, test)]
fn main() -> anyhow::Result<()> {
    // create an import module
    let import = ImportObjectBuilder::new()
        .with_func::<(), ()>("say_hello", say_hello)?
        .build("env")?;

    // let wasm_bytes = wat2wasm(
    //     br#"
    // (module
    //   ;; First we define a type with no parameters and no results.
    //   (type $no_args_no_rets_t (func (param) (result)))

    //   ;; Then we declare that we want to import a function named "env" "say_hello" with
    //   ;; that type signature.
    //   (import "env" "say_hello" (func $say_hello (type $no_args_no_rets_t)))

    //   ;; Finally we create an entrypoint that calls our imported function.
    //   (func $run (type $no_args_no_rets_t)
    //     (call $say_hello))
    //   ;; And mark it as an exported function named "run".
    //   (export "run" (func $run)))
    // "#,
    // )?;

    let wasm_bytes = fs::read("./target/wasm32-wasi/release/wasm_vm_a.wasm")?;

    // loads a wasm module from the given in-memory bytes
    let module = Module::from_bytes(None, wasm_bytes)?;

    // create an executor
    let mut executor = Executor::new(None, None)?;

    // create a store
    let mut store = Store::new()?;

    // register the module into the store
    store.register_import_module(&mut executor, &import)?;

    // register the compiled module into the store and get an module instance
    let extern_instance = store.register_named_module(&mut executor, "extern", &module)?;

    // See also.. The "Vm" approach to spawning wasm, and this fine threading example:
    // https://wasmedge.org/book/en/sdk/rust/concurrent_fib.html

    // get the exported function "run"
    let run = extern_instance
        .func("run")
        .ok_or_else(|| anyhow::Error::msg("Not found exported function named 'run'."))?;

    // run host function
    run.call(&mut executor, params!())?;

    Ok(())
}
