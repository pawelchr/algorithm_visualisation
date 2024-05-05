import React, { useState, useEffect } from 'react';
import { ApiService } from '../services/ApiService';
import './css/SortingVisualization.css';
import BarGraph from './BarGraph';
import SortingMenu from './SortingMenu';

const apiService = new ApiService('http://localhost:8000');

const SortingVisualization: React.FC = () => {
    const [numbers, setNumbers] = useState<number[]>([]); 
    const [width, setWidth] = useState(window.innerWidth * 0.8);
    const [height, setHeight] = useState(window.innerHeight * 0.8);
    const [selectedAlgorithm, setSelectedAlgorithm] = useState<string>('');

    useEffect(() => {
        const handleResize = () => {
            setWidth(window.innerWidth * 0.8);
            setHeight(window.innerHeight * 0.8);
        };

        window.addEventListener('resize', handleResize);
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);

    const handleAlgorithmSelection = (algorithm: string) => {
      setSelectedAlgorithm(algorithm);
      apiService.performSortingAlgorithm(algorithm, numbers)
          .then(sortedNumbers => setNumbers(sortedNumbers))
          .catch(error => console.error(error));
  };

    return (
        <div className='background background-color content'>
            <div>
                <BarGraph numbers={numbers} width={width} height={height} />
            </div>
            <div>
                <SortingMenu setNumbers={setNumbers} handleAlgorithmSelection={handleAlgorithmSelection}/>

            </div>
        </div>
    );
};

export default SortingVisualization;