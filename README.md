
# A simple application for taking notes, saving them as a text file.

### Простое приложение для заметок, сохраняя их в виде текстового файла.

Приложение создано по мотивам статьи на Хабре: [Разрабатываем десктопное приложение для заметок с помощью Tauri (React + Rust)](https://habr.com/ru/companies/timeweb/articles/674342/)
и несколько доработано:
- Приложение не требует настройки клавиш запуска на ярлыке, а выполняет это в коде программы.
- При запуске сразу появляется в трее. У уконки в трее реализовано меню показать/выход.
- При повторном нажатии на комбинацию клавиш или клавиши `Esc` приложение не закрывается, как в оригинале.
- При потери фокуса, например просто выполнив клик ЛКМ на другом приложении или рабочем столе, или при нажатии клавиши `Esc`, приложение прячется в трей.
- Сохранение содержимого окна при потери фокуса, например приложение можно времнно `спрятать` и потом продолжить что уже заводили.
- Смена иконки приложения.

Поскольку приложение при запуске сразу помещается в трэй и на панели задач не видно, - идеально для авто-запуска.

Показ окна ввода выполняется нажатием комбинации клавиш `Ctrl + Shift + Q`.
- Для сохранения введенного текста просто нажимаем `Enter`
- Для того что-бы спрятать окно нажимаем клавишу `Esc`, либо просто переключаясь на любое другое приложение по ЛКМ.

---

## Самостоятельная сборка


В программе используется проект [Tauri](https://tauri.app/), который и позволяет без особых усилий связать бэкенд на Rus с фронтендом на Typescript.
Напрямую сайт к сожалению не доступен, вы знаете что делать... Как минимум можно посетить [страничку проекта на GitHub](https://github.com/tauri-apps/tauri):

Так-же в проекте используется [Vite](https://vite.dev/) для сборки приложения и поддержки `hot reload` - сам отслеживает изменения и пересобирает DEV проект налету.

Соответственно требования для сборки все есть [тут](https://v2.tauri.app/start/prerequisites/#windows)

Коротко для Windows:
1. [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Установленный [Node.js](https://nodejs.org/dist/v22.22.0/node-v22.22.0-x64.msi)
3. Установленный [yarn](https://yarnpkg.com/getting-started/install)
2. WebView2 - уже установлен в Windows 10 и выше. Но можно взять тут [Evergreen Bootstrapper](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section) и установить.
3. VBSCRIPT (for MSI installers) обычно уже установлен, но если получаем ошибку `failed to run light.exe` то:
   - Open Settings → Apps → Optional features → More Windows features
   - Locate VBSCRIPT in the list and ensure it’s checked
   - Click Next and restart your computer if prompted
4. Собственно сам [Rust](https://www.rust-lang.org/tools/install) либо так через winget: `winget install --id Rustlang.Rustup`.
   - Rust поддерживает разные типы toolchain, обычно нам достаточно выполнить `rustup default stable-msvc`

## Рекомендация для IDE

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

====

1. Запуск для разработки:

```
yarn tauri dev
```

2. Создание релиза ( сборка сразу создает установщик в виде `exe` и `msi` ):

```
yarn tauri build
```
