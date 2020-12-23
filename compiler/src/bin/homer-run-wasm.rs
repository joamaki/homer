
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let store = Store::new(&engine);
    let mut linker = Linker::new(&store);

    linker.func("host", "log_i32", |x: i32| println!("{}", x))?;
    linker.func("host", "abort", |x: i32| panic!("abort: {}", x))?;

    let test = Module::from_file(&engine, "/tmp/out.wasm")?;
    let test_inst = linker.instantiate(&test)?;


    //let memory = test_inst.get_memory("")

    let init = test_inst.get_func("init").unwrap();
    init.get0::<()>()?()?;

    let run = test_inst.get_func("$main").unwrap();
    let run = run.get0::<()>()?;
    run()?;

    let deref = test_inst.get_func("deref_i64").unwrap().get0::<i64>()?;
    let result = deref()?;
    println!("result: {}", result);

    let assert_stackempty = test_inst.get_func("assert_stackempty").unwrap().get0::<()>()?;
    assert_stackempty()?;

    let assert_heapempty = test_inst.get_func("assert_heapempty").unwrap().get0::<()>()?;
    assert_heapempty()?;


    Ok(())
}