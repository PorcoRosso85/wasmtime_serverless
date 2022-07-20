#[actix_web::main]
async fn main() -> io::Result<()> {
  HttpServer::new(|| { App::new().service(handler) })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

#[get("/{module_name}")]
async fn handler(module_name: Path<String>) -> impl Responder {
  let wasm_module = format!("{}{}", module_name, ".wasm");
  let val = invoke_wasm_module(wasm_module).expect("");
  HttpResponse::Ok().body(val)
}

fn invoke_wasm_module(module_name: String) -> result::Result<String, wasmtime_wasi::Error> {
  let engine = Engine::default();
  let module = Module::from_file(&engine, module_name)?;
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[])?;
  let exported_run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;
  let res = exported_run.call(&mut store, ())?;
  Ok(res.to_string())
}