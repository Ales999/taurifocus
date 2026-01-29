// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// импорт зависимостей
use std::fs::OpenOptions;
use std::io::Write;

use home::home_dir;

//#[warn(unused_imports)]
//use tauri::menu::Menu;

#[tauri::command]
fn add_task(text: String) {
    // !
    let mut path = home_dir()
     .expect("Ошибка доступа к домашней директории");

    // добавляем в путь название файла для заметок
    path.push("tasks.txt");
    let mut file = OpenOptions::new()
     .create(true)
     .append(true)
     .open(path)
     .expect("Ошибка при открытии файла");

    writeln!(file, "{text}").expect("Ошибка при записи файла");
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let context = tauri::generate_context!();

    //.menu(|app| Menu::default(app)) // Упрощённое API меню
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_task, exit_app])
        .run(context)
        .expect(&format!("Error when running application"));
}
