use tanxium::{
    deno_runtime::{deno_core::ModuleSpecifier, WorkerExecutionMode},
    tanxium::{Tanxium, TanxiumOptions},
};

#[tokio::main(flavor = "current_thread")]
async fn run_js() {
    let main_module = ModuleSpecifier::parse("file:///main.ts").unwrap();
    let mut tanxium = Tanxium::new(TanxiumOptions {
        main_module: main_module.clone(),
        cwd: "/anonymous/path".to_string(),
        extensions: vec![],
        mode: WorkerExecutionMode::None,
    })
    .unwrap();

    // load runtime apis
    match tanxium.load_runtime_api(None).await {
        Err(e) => eprintln!("{}", e.to_string()),
        _ => (),
    };

    // this code is most likely causing a deadlock in the event loop
    let module_code = r#"
    console.log("Hello from JS!");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    console.log("Hello from JS! 2");
    "#
    .to_string();

    // run main module
    match tanxium
        .execute_main_module_code(&main_module, module_code)
        .await
    {
        Err(e) => eprintln!("{}", e.to_string()),
        _ => (),
    };

    // run event loop
    match tanxium.run_event_loop(false).await {
        Err(e) => eprintln!("{}", e.to_string()),
        _ => (),
    };
}

#[tauri::command]
async fn greet() -> String {
    std::thread::spawn(move || run_js()).join().unwrap();

    "Hello from Tauri!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
