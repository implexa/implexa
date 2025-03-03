/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        gray: {
          750: '#2d3748', // Custom color between gray-700 and gray-800
        },
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}