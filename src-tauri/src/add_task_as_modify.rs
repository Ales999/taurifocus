// src-tauri/src/add_task_as_modify.rs

use std::fs::{File, rename};
use std::io::{BufReader, BufWriter, Write, copy};
use std::path::Path;

/// Добавляет строку в начало файла с атомарной заменой
pub fn prepend_line<P: AsRef<Path>>(path: P, line: Option<&str>) -> std::io::Result<()> {
    let line = match line {
        Some(l) if !l.trim().is_empty() => l,
        _ => return Ok(()),
    };

    let path = path.as_ref();

    // Открываем исходный файл для чтения
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    // Создаём временный файл в той же директории
    let temp_path = path.with_file_name(
        format!(".{}.tmp", path.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default())
    );
    let mut temp_file = BufWriter::new(File::create(&temp_path)?);

    // Записываем новую строку в начало
    writeln!(temp_file, "{}", line)?;

    // Копируем содержимое исходного файла (~8 КБ буфер)
    copy(&mut reader, &mut temp_file)?;

    // Синхронизируем на диск (гарантируем запись)
    temp_file.flush()?;
    temp_file.get_ref().sync_all()?;

    // Сохраняем права доступа старого файла
    let metadata = std::fs::metadata(path)?;
    std::fs::set_permissions(&temp_path, metadata.permissions())?;

    // На Windows требуется удалить исходный файл перед заменой
    // https://github.com/rust-lang/rust/issues/88677
    #[cfg(windows)]
    std::fs::remove_file(path)?;

    // Атомарно заменяем исходный файл
    rename(&temp_path, path)?;

    Ok(())
}
