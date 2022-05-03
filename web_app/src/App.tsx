import { useState, useEffect } from 'react'

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={() => console.log("Start playing audio.") }> Play! </button>
        <button onClick={() => console.log("Stop the audio stream.") }> Stop! </button>
      </header>
    </div>
  )
}

export default App
