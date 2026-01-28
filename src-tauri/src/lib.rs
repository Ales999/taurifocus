// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// импорт зависимостей
use std::fs::OpenOptions;
use std::io::Write;

//use tauri::plugin::{Builder as PluginBuilder, TauriPlugin};
//use tauri::Runtime;

//#[warn(unused_imports)]
//use tauri::menu::Menu;

#[tauri::command]
fn add_task(text: String) {
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("../tasks.txt")
    .expect("Ошибка при открытии файла");

  writeln!(file, "{}", text).expect("Ошибка при записи файла");
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}
/*
// Регистрация команд через плагин (обязательно для Tauri 2.x)
fn commands_plugin<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("commands")
        .invoke_handler(tauri::generate_handler![add_task])
        .invoke_handler(tauri::generate_handler![exit_app]) // Регистрируем команду завершения приложения.
        .build()
}
*/


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let context = tauri::generate_context!();
    //.plugin(tauri_plugin_opener::init())
    //.plugin(commands_plugin()) // Замена invoke_handler на плагин
    //.menu(|app| Menu::default(app)) // Упрощённое API меню


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_task, exit_app])
        .run(context)
        .expect(&format!("Error when running application"));

        //.expect("Ошибка при запуске приложения");


        //.invoke_handler(tauri::generate_handler![greet])
        //.run(tauri::generate_context!())
        //.expect("error while running tauri application");
}
