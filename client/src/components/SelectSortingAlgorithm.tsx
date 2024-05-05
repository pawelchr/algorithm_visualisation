import React, { useEffect, useState } from 'react';
import Select, { components, ControlProps } from 'react-select';
import './css/SortingVisualization.css';

type SortingOption = {
  value: string;
  label: string;
};

interface SelectSortingAlgorithmProps {
  handleAlgorithmSelection: (algorithm: string) => void;
}

const ControlComponent = (props: ControlProps<SortingOption, false>) => (
  <div>
    <p className='content'>Select an algorithm</p>
    <components.Control {...props} />
  </div>
);

export default ({ handleAlgorithmSelection }: SelectSortingAlgorithmProps) => {
  const [options, setOptions] = useState<SortingOption[]>([]);

  useEffect(() => {
    fetch('http://127.0.0.1:8000/sort/algorithms')
      .then(response => response.json())
      .then(data => {
        const formattedData = data.map((algorithm: string) => ({
          value: algorithm,
          label: algorithm,
        }));
        setOptions(formattedData);
      });
  }, []);

  return (
    <Select
      isClearable
      components={{ Control: ControlComponent }}
      isSearchable
      name="algo_type"
      options={options}
      className='option-background'
      menuPlacement='auto'
      onChange={(option) => handleAlgorithmSelection(option?.value || '')}
    />
  );
};