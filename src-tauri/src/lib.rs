// src-tauri/src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// импорт зависимостей
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
//use std::io::Read;
//use std::io::Write;
//use std::os::windows::fs::FileExt;

use home::home_dir;
use tauri::{AppHandle, Manager};

#[tauri::command]
fn show_app(app: AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}

#[tauri::command]
fn hide_app(app: AppHandle) {
    // println!("Вызвано `hide_app`");
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
fn add_task(text: String) -> Result<(), String> {
    #[cfg(dev)]
    println!("Вызвано `add_task`");

    // Сразу уберем пробелы в конце строки
    let new_text = text.trim_ascii_end();
    // Проверяем, пустая ли строка
    if new_text.is_empty() {
        return Ok(()); // Возвращаем успешный результат, ничего не делая
    }
    #[cfg(dev)]
    println!("Строка не пустая - добаляем в задачу");

    // Получаем данные о домашней директории
    let home_dir = match home_dir() {
        Some(path) => path,
        None => return Err("Ошибка доступа к домашней директории".to_string()),
    };

    // Добавляем к пути само имя файла
    let mut file_path = home_dir;
    file_path.push("tasks.txt");

    // Проверяем, существует ли файл
    if !file_path.exists() {
        #[cfg(dev)]
        println!("Файл существует");

        // Если файл не существует, создаем новый и записываем туда новую задачу
        let file = OpenOptions::new().write(true).create(true).open(&file_path);

        match file {
            Ok(mut f) => {
                writeln!(
                    f,
                    "{} | {}",
                    chrono::Local::now().format("%d-%m-%Y %H:%M"),
                    new_text.trim_end()
                )
                .map_err(|e| format!("Ошибка при записи нового задания: {}", e))?;
                let _ = f.flush();
            }
            Err(e) => {
                #[cfg(dev)]
                println!("Ошибка при открытии файла: {}", e);
                return Err(format!("Ошибка при открытии файла: {}", e));
            }
        }
    } else {
        #[cfg(dev)]
        println!("Файл существует");

        // Если файл существует, считываем все строки, записываем новую в начало и сохраняем старые
        let file = OpenOptions::new().read(true).write(true).open(&file_path);

        match file {
            Ok(mut f) => {
                let mut old_content = String::new();
                if let Err(e) = f.read_to_string(&mut old_content) {
                    #[cfg(dev)]
                    println!("Ошибка при чтении файла: {}", e);
                    return Err(format!("Ошибка при чтении файла: {}", e));
                }

                // Перемещаем курсор в начало файла
                if let Err(e) = f.seek(SeekFrom::Start(0)) {
                    #[cfg(dev)]
                    println!("Ошибка позиционирования `seek` файла: {}", e);
                    return Err(format!("Ошибка позиционирования файла: {}", e));
                }

                // Обрезаем файл до нулевой длины
                // ! Важно: этот метод вероятно не работает на Windows
                //file.set_len(0)?;
                // Записываем новую задачу

                writeln!(
                    f,
                    "{} | {}",
                    chrono::Local::now().format("%d-%m-%Y %H:%M"),
                    new_text.trim_end()
                )
                .map_err(|e| format!("Ошибка при записи нового задания: {}", e))?;

                if let Err(e) = f.write_all(old_content.as_bytes()) {
                    #[cfg(dev)]
                    println!("Ошибка при записи старых заданий: {}", e);
                    return Err(format!("Ошибка при записи старых заданий: {}", e));
                }
            }
            Err(e) => {
                #[cfg(dev)]
                println!("Ошибка при открытии файла: {}", e);
                return Err(format!("Ошибка при открытии файла: {}", e));
            }
        }
    }

    Ok(())
}

// Команда для выхода из приложения.
#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

#[tauri::command]
fn toggle_app(app: AppHandle) {
    let window = app.get_webview_window("main").unwrap();

    // Проверяем, скрыто ли окно
    if window.is_visible().unwrap_or(false) {
        window.hide().unwrap();
    } else {
        // Вернём окно
        window.unminimize().unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::{
                    menu::{Menu, MenuItem},
                    tray::TrayIconBuilder,
                };
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_shift_q_shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyQ);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            //println!("{:?}", shortcut);
                            //let _ = toggle_app(_app.clone());
                            if shortcut == &ctrl_shift_q_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {}
                                    ShortcutState::Released => {
                                        let _ = toggle_app(_app.clone());
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                // Menu items
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let toggle_i =
                    MenuItem::with_id(app, "toggle", "Показать/Скрыть", true, None::<&str>)?;
                let _menu = Menu::with_items(app, &[&toggle_i, &quit_i])?;

                let _ = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&_menu)
                    .show_menu_on_left_click(true)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "quit" => {
                            #[cfg(dev)]
                            println!("quit menu item was clicked");
                            app.exit(0);
                        }
                        "toggle" => {
                            // Вызываем переключение видимости приложения
                            let _ = toggle_app(app.app_handle().clone());
                        }
                        _ => {
                            #[cfg(dev)]
                            println!("menu item {:?} not handled", event.id);
                        }
                    })
                    .build(app)?;

                // Регистрируем горячую клавишу для переключения
                app.global_shortcut().register(ctrl_shift_q_shortcut)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_app, hide_app, add_task, exit_app, toggle_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
