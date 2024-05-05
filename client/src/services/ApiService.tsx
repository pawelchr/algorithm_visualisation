import axios from 'axios';

class ApiService {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async performSortingAlgorithm(algorithm: string, numbers: number[]): Promise<any> {
    try {
      algorithm = algorithm.toLowerCase();
      console.log(`${this.baseUrl}/sort/${algorithm}`)
      console.log(numbers)
      const response = await axios.post(`${this.baseUrl}/sort/${algorithm}`, { numbers });
      console.log('Sorted numbers:', response.data);
      return response.data;
    } catch (error) {
      console.error('Error performing sorting algorithm:', error);
      throw error;
    }
  }
}

export { ApiService };