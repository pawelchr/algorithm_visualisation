import React from 'react';
import logo from './logo.svg';
import './App.css';
import NavBar from './components/NavBar';
import SortingVisualization from './components/SortingVisualization';

function App() {
  return (
    <div>
      <NavBar />
      <SortingVisualization />
    </div>
  );
}

export default App;
