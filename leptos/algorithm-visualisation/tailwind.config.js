/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        animation: {
          'wall-animation': 'wallAnimation 0.5s ease-out forwards',
          'visited-animation': 'visitedAnimation 1.5s ease-out forwards',
          'shortest-path': 'shortestPathAnimation 0.5s ease-out forwards',
          'finish-visited-animation': 'finishVisitedAnimation 1.5s ease-out forwards',
        },
        keyframes: {
          wallAnimation: {
            '0%': { transform: 'scale(0.4)', backgroundColor: '#021c33', borderRadius: '30%' },
            '50%': { transform: 'scale(0.6)', backgroundColor: '#021c33', borderRadius: '20%' },
            '75%': { transform: 'scale(0.8)', backgroundColor: '#021c33', borderRadius: '10%' },
            '100%': { transform: 'scale(1)', backgroundColor: '#021c33', border: '1px solid #021c33' },
          },
          visitedAnimation: {
            '0%': { transform: 'scale(0.3)', backgroundColor: 'rgba(255, 0, 255)', borderRadius: '100%' },
            '50%': { transform: 'scale(0.5)', backgroundColor: 'rgba(10, 55, 95, 0.75)', borderRadius: '75%' },
            '75%': { transform: 'scale(0.7)', backgroundColor: 'rgba(20, 110, 140, 0.75)', borderRadius: '50%' },
            '100%': { transform: 'scale(1)', backgroundColor: 'rgba(30, 165, 185, 0.75)' },
          },
          shortestPathAnimation: {
            '0%': { transform: 'scale(0.3)', backgroundColor: 'rgba(255, 0, 255, 0.75)', borderRadius: '100%' },
            '50%': { transform: 'scale(0.5)', backgroundColor: 'rgba(255, 0, 255, 0.75)', borderRadius: '75%' },
            '75%': { transform: 'scale(0.7)', backgroundColor: 'rgba(255, 0, 255, 0.75)', borderRadius: '50%' },
            '100%': { transform: 'scale(1)', backgroundColor: 'rgba(255, 0, 255, 0.75)' },
          },
          finishVisitedAnimation: {
            '0%': { transform: 'scale(0.5)', backgroundColor: 'red', borderRadius: '50%' },
            '50%': { transform: 'scale(0.8)', backgroundColor: 'red', borderRadius: '25%' },
            '75%': { transform: 'scale(1.2)', backgroundColor: 'red', borderRadius: '20%' },
            '100%': { transform: 'scale(1)', backgroundColor: 'red' },
          },
        },
      },
    },
    plugins: [],
  }