// add_task_as_modify.rs

use std::fs::{File, rename};
use std::io::{BufReader, BufWriter, Write, copy};
use std::path::Path;

pub fn prepend_line<P: AsRef<Path>>(path: P, line: &str) -> std::io::Result<()> {
    let path = path.as_ref();

    // 1. Открываем исходный файл для чтения
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    // 2. Создаём временный файл в той же директории
    let temp_path = path.with_file_name(format!(
        ".{}.tmp",
        path.file_name().unwrap_or_default().to_string_lossy()
    ));
    let mut temp_file = BufWriter::new(File::create(&temp_path)?);

    // 3. Записываем новую строку в начало
    writeln!(temp_file, "{}", line)?;

    // 4. Копируем содержимое исходного файла блоками (буфер ~8 КБ)
    copy(&mut reader, &mut temp_file)?;

    // 5. Синхронизируем на диск (гарантируем запись)
    temp_file.flush()?;
    temp_file.get_ref().sync_all()?;

    // 6. Сохраним права доступа у старого файла и применим к новому.
    let metadata = std::fs::metadata(path)?;
    std::fs::set_permissions(&temp_path, metadata.permissions())?;

    // 6.1 - Специально для Windows платформы.
    #[cfg(windows)]
    std::fs::remove_file(path)?;

    // 7. Атомарно заменяем исходный файл
    rename(&temp_path, path)?;

    Ok(())
}
