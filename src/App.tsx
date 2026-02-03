import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import "./App.css";

function App() {
  const [text, setText] = useState('');

  const addTask = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    switch (e.key) {
      case 'Enter':
        // Если там не только пробел(ы), то добавляем задачу
        if (text.trim().length > 0) {
          try {
            await invoke('add_task', { text });
            setText('');
          } catch (e) {
            console.error(e);
          }
        } else {
          // Если там одни пробелы то очищаем ввод
          setText('');
        }
        break;
      case 'Escape':
        try {
          await invoke('hide_app');
        } catch (e) {
          console.error('Ошибка скрытия окна', e)
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
        const unlisten = await getCurrentWindow().listen('tauri://blur', () => {
          invoke('hide_app');
        });
        return unlisten;
      } catch (e) {
        console.error('Ошибка инициализации слушателя blur:', e);
        return () => { };
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


  /*
  // Данный эффект сработает когда приложение скрывается (hiden)
  // ! Это вероятно не работает, - проверить дополнительно.
  // TODO: Можно сохранять во временное место, и вернуть обратно при открытии приложения, а не очищать
  useEffect(() => {
    (async () => {
      const unlistenCloseReq = await getCurrentWindow().listen('tauri://closeRequested', () => {
        setText('');
      });
      return () => unlistenCloseReq();
    })()
  }, []);
  */

  return (
    <div>
      <input
        type='text'
        className='w-150 h-15 px-4 bg-gray-800 text-2xl text-green-600 rounded-sm'
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={addTask}
        autoFocus
      />
    </div>
  )
}

export default App
