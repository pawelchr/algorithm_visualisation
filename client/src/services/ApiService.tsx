import axios from 'axios';

class ApiService {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async performSortingAlgorithm(algorithm: string, numbers: number[]): Promise<any> {
    try {
      const response = await axios.post(`${this.baseUrl}/sort/${algorithm}`, { numbers });

      return response.data;
    } catch (error) {
      console.error('Error performing sorting algorithm:', error);
      throw error;
    }
  }
}

export { ApiService };