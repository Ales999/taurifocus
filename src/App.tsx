import { useState } from "react";
//import reactLogo from "./assets/react.svg";
//import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [text, setText] = useState('')

  const addTask = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      console.log(text)
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
