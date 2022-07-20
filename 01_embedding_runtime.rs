use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
  let engine = Engine::default();
  let module = Module::from_file(&engine, "hello.wat")?;
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[])?;
  let exported_run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;
  let res = exported_run.call(&mut store, ())?;
  
  println!("WebAssembly says - {}", res);
  Ok(())
}