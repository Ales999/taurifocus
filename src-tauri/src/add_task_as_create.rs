// src-tauri/src/add_task_as_create.rs

use std::fs::OpenOptions;
use std::path::Path;
use std::io::{Write};

// Создаем файл и записываем в него строку.
pub fn created_line<P: AsRef<Path>>(path: P, line: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path);

    match file {
        Ok(mut f) => {
            // Проверяем результат записи
            writeln!(f, "{}", line)?;
            // Проверяем результат сброса буфера
            f.flush()?;

            Ok(())
        }
        Err(e) => {
            #[cfg(dev)]
            println!("Ошибка при открытии файла: {}", e);
            Err(e)
        }
    }
}
