// src-tauri/src/add_task_as_create.rs

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// Создаем файл и записываем в него строку.
pub fn created_line<P: AsRef<Path>>(path: P, line: Option<&str>) -> std::io::Result<()> {
    // Если строка пустая то просто выходим.
    let line = match line {
        Some(l) => l,
        None => return Ok(()),
    };

    // Если строка состоит только из пробелов тоже просто выходим.
    if line.trim().is_empty() {
        return Ok(());
    }

    // Создаем новый файл.
    let mut file = OpenOptions::new().write(true).create(true).open(&path)?;

    writeln!(file, "{}", line)?; // Проверяем результат записи

    Ok(())
}
