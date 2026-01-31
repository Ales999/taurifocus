// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// импорт зависимостей
use std::fs::OpenOptions;
use std::io::Write;

use home::home_dir;

//#[warn(unused_imports)]
//use tauri::menu::Menu;

#[tauri::command]
fn add_task(text: String) {
    // Определяем путь к файлу
    let mut path = home_dir()
        .expect("Ошибка доступа к домашней директории");
    path.push("tasks.txt");

    // Читаем текущее содержимое (если файл не существует – пустая строка)
    let old_content = std::fs::read_to_string(&path).unwrap_or_default();

    // Получаем текущую дату/время в нужном формате
    // формат: день-месяц-год часы:минуты
    let timestamp = chrono::Local::now()
        .format("%d-%m-%Y %H:%M")
        .to_string();

    let new_line = format!("{} {}", timestamp, text);

    // Открываем файл для записи, обнуляя его
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)   // удаляем старое содержимое
        .open(path)
        .expect("Ошибка при открытии файла");

    // Записываем новую строку, затем старое содержимое
    writeln!(file, "{}", new_line)
        .expect("Ошибка при записи нового задания");
    file.write_all(old_content.as_bytes())
        .expect("Ошибка при записи старого содержимого");
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
