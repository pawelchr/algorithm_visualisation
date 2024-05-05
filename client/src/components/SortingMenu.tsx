import React, { useState, useEffect } from 'react';
import './css/SortingVisualization.css';
import SelectSortingAlgorithm from './SelectSortingAlgorithm';
import ArrayToSort from './ArrayToSort';
import Button from 'react-bootstrap/Button';

interface SortingMenuProps {
    setNumbers: React.Dispatch<React.SetStateAction<number[]>>;
    handleAlgorithmSelection: (algorithm: string) => void;
}



const SortingMenu: React.FC<SortingMenuProps> = ({ setNumbers, handleAlgorithmSelection }) => {

    const numbers = [1, 2, 3, 4, 5, 6, 7]; // Example numbers
  
  return (
    <div className='sorting-menu'>
        <div className='sorting-menu'>
            <ArrayToSort setNumbers={setNumbers}/>
        </div>
        <div>
            <SelectSortingAlgorithm handleAlgorithmSelection={handleAlgorithmSelection}/>
        </div>
        <div>
            <Button variant="secondary" className='button'>Previous Step</Button>
            <Button variant="primary" className='button'>Next Step</Button>
        </div>
    </div>
  );
};

export default SortingMenu;