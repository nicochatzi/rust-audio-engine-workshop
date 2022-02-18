import React, { useEffect, useState } from 'react';
import init, { AudioEngine } from '../../audio_engine/pkg';

const WebBasedEngine: React.FC = () => {
  const [audioEngine, setAudioEngine] = useState<AudioEngine>();
  const [gain, setGain] = useState<number>();

  useEffect(() => {
    init()
      .catch(console.error)
      .then(() => setAudioEngine(new AudioEngine(44100, 2)));
  }, []);

  return (
    <div className="App-header">
      <button onClick={() => setAudioHandle(audioEngine.play())}>Play!</button>
      <button onClick={() => audioHandle.free()}>Stop!</button>
    </div>
  );
};

export default WebBasedEngine;
