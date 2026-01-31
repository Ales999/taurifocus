// src-tauri/src/lib.rs
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// импорт зависимостей
use std::fs::OpenOptions;
use std::io::Write;
use tauri::{AppHandle, Manager};
use home::home_dir;

#[tauri::command]
fn show_app(app: AppHandle) {
    println!("Вызвано `show_app`");
    let window = app.get_webview_window("main").unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}

#[tauri::command]
fn hide_app(app: AppHandle) {
    println!("Вызвано `hide_app`");
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
fn add_task(text: String) {
    println!("Вызвано `add_task`");

    let mut path = home_dir().expect("Ошибка доступа к домашней директории");
    path.push("tasks.txt");

    let old_content = std::fs::read_to_string(&path).unwrap_or_default();
    let timestamp = chrono::Local::now().format("%d-%m-%Y %H:%M").to_string();
    let new_line = format!("{} {}", timestamp, text);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Ошибка при открытии файла");

    writeln!(file, "{}", new_line).expect("Ошибка при записи нового задания");
    file.write_all(old_content.as_bytes()).expect("Ошибка при записи старого содержимого");
}

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
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
                use tauri::{
                    menu::{Menu, MenuItem},
                    tray::TrayIconBuilder,
                };

                let ctrl_shift_q_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyQ);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
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
                    .build()
                )?;

                // Menu items
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let toggle_i = MenuItem::with_id(app, "toggle", "Показать/Скрыть", true, None::<&str>)?;
                let _menu = Menu::with_items(app, &[&toggle_i, &quit_i ])?;

                let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&_menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "toggle" => {
                        // Вызываем переключение видимости приложения
                         let _ = toggle_app(app.app_handle().clone());
                    }
                    _ => {
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
            show_app,
            hide_app,
            add_task,
            exit_app,
            toggle_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
