import React, { useState, useEffect } from 'react';
import './css/SortingVisualization.css';
import SelectSortingAlgorithm from './SelectSortingAlgorithm';
import { SortedNumbersResponse } from './SortingVisualization';
import ArrayToSort from './ArrayToSort';
import Button from 'react-bootstrap/Button';

interface SortingMenuProps {
    setNumbers: React.Dispatch<React.SetStateAction<number[]>>;
    handleAlgorithmSelection: (algorithm: string) => void;
    setStep: React.Dispatch<React.SetStateAction<number>>;
    step: number;
    maxStep: number;
    selectedAlgorithm: string;
}

const SortingMenu: React.FC<SortingMenuProps> = ({ setNumbers, handleAlgorithmSelection, setStep, step, maxStep, selectedAlgorithm}) => {
  return (
    <div className='sorting-menu'>
        <div className='sorting-menu'>
            <ArrayToSort setNumbers={setNumbers} setStep={setStep} selectedAlgorithm={selectedAlgorithm}/>
        </div>
        <div>
            <SelectSortingAlgorithm handleAlgorithmSelection={handleAlgorithmSelection}/>
        </div>
        <div>
            <Button variant="secondary" className='button' onClick={() => setStep(prevStep => prevStep > 0 ? prevStep - 1 : 0)} disabled={step === 0}>Previous Step</Button>
            <Button variant="primary" className='button' onClick={() => setStep(prevStep => prevStep + 1)} disabled={step > maxStep - 1}>Next Step</Button>
        </div>
    </div>
  );
};

export default SortingMenu;