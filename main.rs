use anyhow::Result;
use wasmtime::*;

#[actix_web::main]
fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(handler))
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
    let mut linker = Linker::new(&engine);
    wasmtime_was::add_to_linker(&mut linker, |s| s)?;

    let stdout_buf: Vec<8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());

    let wasi = WasiCtxBuilder::new().stdout().build();
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(&engine, module_name)?;
    linker.module(&mut store, &module_name, &module)?;

    let instance = linker.instance(&mut store, &module)?;
    let instance_main = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;
    instance_main.call(&mut store, ())?;

    let mut buffer: Vec<u8> = Vec::new();
    stdout_mutex
        .read()
        .unwrap()
        .iter()
        .for_each(|i| buffer.push(*i));
    let s = String::from_utf8(buffer)?;
    Ok(s)
}

use sudoku::Sudoku;
fn main() {
    let puzzle_line = std::env::var("puzzle").unwrap();
    let sudoku = Sudoku::from_str_line(&puzzle_line).unwrap();
    if let Some(solution) = sudoku.solve_unique() {
        let str = solution.to_str_line().to_string();
        for i in 0..9 {
            println!("{}", str[(i * 9)..(i * 9 + 9)].to_string());
        }
    } else {
        println!("failed to solve");
    }
}
