// src-tauri/src/add_task_as_create.rs

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Создаем файл и записываем в него строку.
pub fn created_line<P: AsRef<Path>>(path: P, line: &str) -> std::io::Result<()> {
    // Создаем новый файл.
    let mut file = OpenOptions::new().write(true).create(true).open(&path)?;

    writeln!(file, "{}", line)?; // Проверяем результат записи

    Ok(())
}
