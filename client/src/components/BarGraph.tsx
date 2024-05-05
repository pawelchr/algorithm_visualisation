import React, { useState } from 'react';
import './css/BarGraph.css';
import { BarChart } from '@mui/x-charts/BarChart';

interface BarGraphProps {
  numbers: number[];
  width: number;
  height: number;
}


const BarGraph: React.FC<BarGraphProps> = ({ numbers, width, height }) => {
  const xAxisData = numbers.map((_, index) => [``]);
  const seriesData = [{ data: numbers }];
  const [key, setKey] = useState(0);

  const forceReload = () => {
    setKey(prevKey => prevKey + 1);
  };

  return (
    <div key={key}>
      <BarChart
        xAxis={[{ scaleType: 'band', data: xAxisData }]}
        series={seriesData}
        width={width}
        height={height}
        tooltip={{ trigger: 'item' }}
      />
    </div>
  );
};

export default BarGraph;