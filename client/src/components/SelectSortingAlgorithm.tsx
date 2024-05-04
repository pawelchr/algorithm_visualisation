import React, { useEffect, useState } from 'react';
import Select, { components, ControlProps } from 'react-select';

type SortingOption = {
  value: string;
  label: string;
};

const ControlComponent = (props: ControlProps<SortingOption, false>) => (
  <div>
    <p className='content'>Select an algorithm</p>
    <components.Control {...props} />
  </div>
);

export default () => {
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
      className='option-color'
    />
  );
};