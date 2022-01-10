import React, { useEffect, useState } from 'react';
import './App.css';
import init, { play } from '../../audio_engine/pkg';

const App: React.FC = () => {
  const [audioEngine, setAudioEngine] = useState<any>();
  const [audioHandle, setAudioHandle] = useState<any>();

  useEffect(() => {
    init()
      .catch(console.error)
      .then(() => setAudioEngine({ play }));
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={() => setAudioHandle(audioEngine.play())}> Play! </button>
        <button onClick={() => audioHandle.free()}> Stop! </button>
      </header>
    </div>
  );
};

export default App;
