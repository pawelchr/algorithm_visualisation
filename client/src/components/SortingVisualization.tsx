import React, { useState, useEffect } from 'react';
import { ApiService } from '../services/ApiService';
import './css/SortingVisualization.css';
import BarGraph from './BarGraph';
import SortingMenu from './SortingMenu';

const apiService = new ApiService('http://localhost:8000');

export type SortedNumbersResponse = {
    array_accesses: number;
    duration: {
      nanos: number;
      secs: number;
    };
    result: number[];
    results_length: number;
    status: string;
  };

const SortingVisualization: React.FC = () => {
    const [numbers, setNumbers] = useState<number[]>([]);
    const [sortedNumbers, setSortedNumbers] = useState<SortedNumbersResponse | null>(null);
    const [width, setWidth] = useState(window.innerWidth * 0.8);
    const [height, setHeight] = useState(window.innerHeight * 0.8);
    const [selectedAlgorithm, setSelectedAlgorithm] = useState<string>('');
    const [step, setStep] = useState<number>(0);
    const [maxStep, setMaxStep] = useState<number>(0);

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
            .then(sortedNumbersResult => {
                setSortedNumbers(sortedNumbersResult);
                console.log('sortedNumbersResult.result', sortedNumbersResult.result.length);
                console.log('sortedNumbersResult.results_length', sortedNumbersResult.results_length);
                let maxStep = sortedNumbersResult.result.length / sortedNumbersResult.results_length
                setMaxStep(maxStep);
                console.log('maxstep', maxStep);
            })
            .catch(error => console.error(error));
    };

    return (
        <div className='background background-color content'>
            <div>
                <BarGraph sortedNumbersResponse={sortedNumbers} width={width} height={height} step={step}/>
            </div>
            <div>
                <SortingMenu setNumbers={setNumbers} handleAlgorithmSelection={handleAlgorithmSelection} setStep={setStep} step={step} maxStep={maxStep} selectedAlgorithm={selectedAlgorithm}/>
                {selectedAlgorithm && <p className='algorithm-selected'>Selected algorithm: {selectedAlgorithm}</p>}
            </div>
        </div>
    );
};

export default SortingVisualization;