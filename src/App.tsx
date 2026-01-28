import React, { useState } from "react";
//import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
//import { invoke, process } from '@tauri-apps/api'

import "./App.css";

function App() {
  const [text, setText] = useState('')

  const addTask = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    //if ( e && !isNaN(+text)) return;

    switch (e.key) {
    // при нажатии `Enter` вызываем `add_task` с текстом заметки
    case 'Enter':
      try {
        await invoke('add_task', { text })
        setText('')
      } catch (e) {
        console.error(e)
      }
      break
    // при нажатии `Esc` завершаем процесс
    case 'Escape':
      await invoke('exit_app')
      return
    default:
      return
  }

  }

  return (
    <input
      type='text'
      className='w-150 h-15 px-4 bg-gray-800 text-2xl text-green-600 rounded-sm'
      value={text}
      onChange={(e) => setText(e.target.value)}
      onKeyDown={addTask}
    />
  )
}

export default App
