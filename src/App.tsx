import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";

function App() {
  const [text, setText] = useState("");
  // Для вывода ошибки
  const [error, setError] = React.useState<string | null>(null);

  const addTask = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    switch (e.key) {
      case "Enter":
        // setText('-----------------');
        // Если там не только пробел(ы), то добавляем задачу
        if (text.trim().length > 0) {
          try {
            await invoke("add_task", { text })
              .then(() => {
                console.log("Задача добавлена");
                // Раз всё успешно, очищаем ввод
                setText("");
                setError(null); // Сбрасываем ошибку при успехе
              })
              .catch((err) => {
                setText(""); // Очищаем ввод, если ошибка
                setError(String(err)); // Сохраняем ошибку в состояние для отображения
              });
          } catch (e) {
            console.error(e);
          }
        } else {
          // Если там одни пробелы то очищаем ввод
          setText("");
        }
        break;
      case "Escape":
        try {
          await invoke("hide_app");
        } catch (e) {
          console.error("Ошибка скрытия окна", e);
        }
        return;
      default:
        return;
    }
  };

  // Данный эффект сработает когда приложение теряет фокус - мы его прячем в трэй
  useEffect(() => {
    const initBlurListener = async () => {
      try {
        const unlisten = await getCurrentWindow().listen("tauri://blur", () => {
          invoke("hide_app");
        });
        return unlisten;
      } catch (e) {
        console.error("Ошибка инициализации слушателя blur:", e);
        return () => {};
      }
    };

    const unlistenPromise = initBlurListener();
    // Очитка компоненнта - используем then() вместо await, так как useEffect не может быть async
    unlistenPromise.then((unlisten) => {
      return () => {
        unlisten(); // Вызываем функцию отмены
      };
    });
  }, []);

  // Данный эффект сработает когда окно закрывается а не прячется (например закрыли через крестик окна)
  // ! Это вероятно не работает, - проверить дополнительно.
  // TODO: Можно сохранять во временное место, и вернуть обратно при открытии приложения, а не очищать
  useEffect(() => {
    const initCloseReqListener = async () => {
      try {
        const unlistenCloseReq = await getCurrentWindow().listen(
          "tauri://closeRequested",
          () => {
            if (text.length > 0) {
              // Если что-то осталось в поле ввода
            }
            setText("");
          },
        );
        return unlistenCloseReq;
      } catch (e) {
        console.error("Ошибка инициализации слушателя closeRequested:", e);
        return () => {};
      }
    };
    const unlistenCloseReqPromise = initCloseReqListener();
    // Очистка компонента
    unlistenCloseReqPromise.then((unlistenCloseReq) => {
      return () => {
        unlistenCloseReq();
      };
    });
  }, []);

  return (
    <div className="w-full max-w-max mx-auto">
      <input
        type="text"
        className="w-[600px] h-[60px] px-4 bg-gray-800 text-2xl text-green-600 rounded-sm input-error"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={addTask}
        autoFocus
        placeholder={error ? `Ошибка: ${error}` : ``}
      />
    </div>
  );
}

export default App;
