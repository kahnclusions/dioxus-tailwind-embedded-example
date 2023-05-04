/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.rs',
    './index.html',
    './src/**/*.html',
    './src/**/*.css'
  ],
  theme: {
    fontFamily: {
      sans: ['Noto Sans', 'Arial'],
      serif: ['Noto Serif', 'Georgia'],
      display: ['Underdog']
    },
    extend: {
      colors: {
        black: "#4c4f69",
        white: "#eff1f5",
      }
    },
  },
  plugins: [],
}
