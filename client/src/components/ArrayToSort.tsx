import React, { useState, useEffect } from 'react';
import './css/SortingVisualization.css';

interface ArrayToSortProps {
    setNumbers: React.Dispatch<React.SetStateAction<number[]>>;
    setStep: React.Dispatch<React.SetStateAction<number>>;
    selectedAlgorithm: string;
}

const ArrayToSort: React.FC<ArrayToSortProps> = ({ setNumbers, setStep, selectedAlgorithm }) => {
    const [input, setInput] = useState('');
    const [error, setError] = useState('');

    useEffect(() => {
      setStep(1);
  }, [selectedAlgorithm]);

    const handleInput = () => {
        let separator;
        if (input.includes(',')) {
            if (input.includes(', ')) {
                separator = ', ';
            } else {
                separator = ',';
            }
        } else {
            separator = ' ';
        }
    
        const numArray = input.split(separator).map(Number);
        if (numArray.includes(NaN)) {
            setError('Invalid input. Please enter valid numbers.');
        } else {
            setNumbers(numArray);
            setStep(1);
            setError('');
        }
    };

    const handleKeyPress = (event: React.KeyboardEvent) => {
      if (event.key === 'Enter') {
        event.preventDefault();
        handleInput();
      }
    };

  return (
    <div>
      <input
        className='input-style'
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onBlur={handleInput}
        onKeyDown={handleKeyPress}
        placeholder="Enter numbers to sort"
      />
      {error && <p className='error-message'>{error}</p>}
    </div>
  );
};

export default ArrayToSort;