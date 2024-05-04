import React, { useState, useEffect } from 'react';
import { ApiService } from '../services/ApiService';
import './css/SortingVisualization.css';
import BarGraph from './BarGraph';
import SelectSortingAlgorithm from './SelectSortingAlgorithm';

const apiService = new ApiService('http://localhost:8000');

const SortingVisualization: React.FC = () => {

    const numbers = [1, 2, 3, 4, 5, 6, 7]; // Example numbers
    const width = 500; // Example width
    const height = 300; // Example height
  
  return (
    <div className='background background-color'>
        <div className='content'>
            <BarGraph numbers={numbers} width={width} height={height} />
        </div>
        <div>
            <SelectSortingAlgorithm />
        </div>
    </div>
  );
};

export default SortingVisualization;