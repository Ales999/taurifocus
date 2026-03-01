// src-tauri/src/lib.rs

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use home::home_dir;
use tauri::menu::PredefinedMenuItem;
use tauri::{AppHandle, Manager, WebviewWindow};

// Код работы с файлом
mod add_task_as_create;
mod add_task_as_modify;

// Имя файла в домашнем каталоге для заметок
const TASK_FILE_NAME: &str = "tfocus_tasks.txt";
// Имя основного окна
const MAIN_WINDOW_LABEL: &str = "main";

/// Вспомогательная функция для получения основного окна
fn get_main_window(app: &AppHandle) -> Result<WebviewWindow, tauri::Error> {
    app.get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or(tauri::Error::WindowNotFound)
}

/// Показать окно приложения
#[tauri::command]
fn show_app(app: AppHandle) -> Result<(), tauri::Error> {
    let window = get_main_window(&app)?;

    window.unminimize()?;
    window.show()?;
    window.set_focus()?;

    Ok(())
}

/// Скрыть окно приложения
#[tauri::command]
fn hide_app(app: AppHandle) -> Result<(), tauri::Error> {
    let window = get_main_window(&app)?;
    window.hide()?;
    Ok(())
}

/// Добавить задачу в файл
#[tauri::command]
fn add_task(text: String) -> Result<(), String> {
    #[cfg(debug_assertions)]
    println!("Вызвано `add_task`");

    let new_text = text.trim_ascii_end();
    if new_text.is_empty() {
        return Ok(());
    }

    #[cfg(debug_assertions)]
    println!("Строка не пустая - добавляем в задачу");

    let timestamp = chrono::Local::now().format("%d-%m-%Y %H:%M").to_string();
    let formatted_text = format!("{} | {}", timestamp, new_text);

    let home_dir = home_dir().ok_or("доступа к домашней директории")?;
    let file_path = home_dir.join(TASK_FILE_NAME);

    if file_path.exists() {
        #[cfg(debug_assertions)]
        println!("Файл существует - модифицируем");

        add_task_as_modify::prepend_line(file_path, Some(&formatted_text))
            .map_err(|e| format!("операции с файлом: {}", e))?;
    } else {
        #[cfg(debug_assertions)]
        println!("Файл не существует - создаём");

        add_task_as_create::create_with_line(file_path, Some(&formatted_text))
            .map_err(|e| format!("при создании/записи файла: {}", e))?;
    }

    Ok(())
}

/// Выйти из приложения
#[tauri::command]
fn exit_app(app: AppHandle) {
    app.exit(0);
}

/// Переключить видимость окна
#[tauri::command]
fn toggle_app(app: AppHandle) -> Result<(), tauri::Error> {
    let window = get_main_window(&app)?;

    if window.is_visible().unwrap_or(false) {
        window.hide()?;
    } else {
        window.unminimize()?;
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
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
                        .with_handler(move |app, shortcut, event| {
                            if shortcut == &ctrl_shift_q_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        let _ = show_app(app.clone());
                                    }
                                    ShortcutState::Released => {}
                                }
                            }
                        })
                        .build(),
                )?;

                let toggle_i =
                    MenuItem::with_id(app, "show_input", "Показать окно", true, None::<&str>)?;
                let separator_i = PredefinedMenuItem::separator(app)?;
                let quit_i = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&toggle_i, &separator_i, &quit_i])?;

                let _ = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .show_menu_on_left_click(true)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show_input" => {
                            if let Err(_e) = show_app(app.app_handle().clone()) {
                                #[cfg(debug_assertions)]
                                eprintln!("Ошибка при показе окна: {}", _e);
                            }
                        }
                        "quit" => {
                            #[cfg(debug_assertions)]
                            println!("quit menu item was clicked");
                            app.exit(0);
                        }
                        _ => {
                            #[cfg(debug_assertions)]
                            eprintln!("menu item {:?} not handled", event.id);
                        }
                    })
                    .build(app)?;

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
