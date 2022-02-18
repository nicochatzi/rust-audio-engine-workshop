import React, { useEffect, useState } from 'react';
import init, { play } from '../../audio_engine/pkg';

const StandaloneEngine: React.FC = () => {
  const [audioEngine, setAudioEngine] = useState<any>();
  const [audioHandle, setAudioHandle] = useState<any>();

  useEffect(() => {
    init()
      .catch(console.error)
      .then(() => setAudioEngine({ play }));
  }, []);

  return (
    <div className="App-header">
      <button onClick={() => setAudioHandle(audioEngine.play())}>Play!</button>
      <button onClick={() => audioHandle.free()}>Stop!</button>
    </div>
  );
};

export default StandaloneEngine;
