// src-tauri/src/add_task_as_create.rs

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Создаёт файл и записывает в него строку
pub fn create_with_line<P: AsRef<Path>>(path: P, line: Option<&str>) -> std::io::Result<()> {
    let line = match line {
        Some(l) if !l.trim().is_empty() => l,
        _ => return Ok(()),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}
