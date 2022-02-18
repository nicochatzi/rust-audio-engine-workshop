import React from 'react';
import './App.css';
import StandaloneEngine from './StandaloneEngine';
import WebBasedEngine from './WebBasedEngine';

const App: React.FC = () => (
  <div className="App">
    <StandaloneEngine />
  </div>
);

export default App;
