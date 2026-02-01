
# A simple application for taking notes, saving them as a text file.

### Простое приложение для заметок, сохраняя их в виде текстового файла.

Приложение создано по мотивам статьи на Хабре: [Разрабатываем десктопное приложение для заметок с помощью Tauri (React + Rust)](https://habr.com/ru/companies/timeweb/articles/674342/)
и несколько доработано.

Приложение при запуске сразу помещается в трэй и на панели задач не видно, - идеально для авто-запуска.

После чего просто ожидает комбинации клавиш `Ctrl + Shift + Q` для показа небольшого поля ввода, прямо по середине экрана.
Набираем свою заметку, нажимаем Enter - он запишется в файл и окно останется открытым и можно будет его скрыть с помощью клавиш `Ctrl + Shift + Q`.
Либо по клавише `Esc` либо просто переключаясь на любое другое приложение.

Так-же, для тех кто привык все делать мышкой, то можно кликнуть ЛКМ на иконку в трее, и выбрать соответствующее меню.

---

## Самостоятельная сборка


В программе используется проект [Tauri](https://tauri.app/), который и позволяет без особых усилий связать бэкенд на Rus с фронтендом на Typescript.
Напрямую сайт к сожалению не доступен, вы знаете что делать... Как минимум можно посетить [страничку проекта на GitHub](https://github.com/tauri-apps/tauri):

Так-же в проекте используется [Vite](https://vite.dev/) для сборки приложения и поддержки `hot reload` - сам отслеживает изменения и пересобирает DEV проект налету.

Соответственно требования для сборки все есть [тут](https://v2.tauri.app/start/prerequisites/#windows)

Коротко для Windows:
1. [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. WebView2 - уже установлен в Windows 10 и выше. Но можно взять тут [Evergreen Bootstrapper](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section) и установить.
3. VBSCRIPT (for MSI installers) обычно уже установлен, но если получаем ошибку `failed to run light.exe` то:
   - Open Settings → Apps → Optional features → More Windows features
   - Locate VBSCRIPT in the list and ensure it’s checked
   - Click Next and restart your computer if prompted
4. Собственно сам [Rust](https://www.rust-lang.org/tools/install) либо так: `winget install --id Rustlang.Rustup`.
   - Rust поддерживает разные типы toolchain, обычно нам достаточно выполнить `rustup default stable-msvc`


## Рекомендация для IDE

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

====

Run Dev:

```
yarn tauri dev
```

Build Relase App, and install setup with exe and msi:

```
yarn tauri build
```
