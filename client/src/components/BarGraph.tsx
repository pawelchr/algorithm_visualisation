import React, { useState, useEffect } from 'react';
import './css/BarGraph.css';
import { SortedNumbersResponse } from './SortingVisualization';
import { BarChart } from '@mui/x-charts/BarChart';

interface BarGraphProps {
  sortedNumbersResponse: SortedNumbersResponse | null;
  width: number;
  height: number;
  step: number;
}

const NoDataComponent: React.FC = () => (
  <div>
    <h2>No data available</h2>
    <p>Please select a different dataset or check back later.</p>
  </div>
);

const BarGraph: React.FC<BarGraphProps> = ({ sortedNumbersResponse, width, height, step }) => {
  const [key, setKey] = useState(0);
  const [startIndex, setStartIndex] = useState(0);
  const [endIndex, setEndIndex] = useState(0);
  const [previousStep, setPreviousStep] = useState(0);

  let numbers = sortedNumbersResponse? sortedNumbersResponse.result : [];
  numbers = Array.isArray(numbers)? numbers : [];
  
  useEffect(() => {
    if (sortedNumbersResponse) {
      if (step > previousStep) {
        setStartIndex(endIndex);
        setEndIndex(step * sortedNumbersResponse.results_length);
      } else if (step < previousStep) {
        setEndIndex(startIndex);
        setStartIndex(startIndex - sortedNumbersResponse.results_length);
      }
      setPreviousStep(step);
    }
  }, [sortedNumbersResponse, step]);
  numbers = numbers.slice(startIndex, endIndex);
  console.log(startIndex);
  console.log(endIndex);

  const xAxisData = numbers.map((_, index) => [``]);
  const seriesData = [{ data: numbers }];

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