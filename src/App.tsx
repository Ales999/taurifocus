import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import "./App.css";

function App() {
  const [text, setText] = useState('');

  const addTask = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    switch (e.key) {
      case 'Enter':
        try {
          await invoke('add_task', { text });
          setText('');
        } catch (e) {
          console.error(e);
        }
        break;
      case 'Escape':
        await invoke('hide_app');
        return;
      default:
        return;
    }
  };


  useEffect(() => {
    (async () => {
      const unlisten = await getCurrentWindow().listen('tauri://blur', () => {
        invoke('hide_app');
      });
      return () => {
        unlisten();
      };
    })();
  }, []);

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
