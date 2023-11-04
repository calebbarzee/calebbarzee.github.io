/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        'primary': '#eae0d5', // Light Background
        'secondary': '#e4e6eb', // Light Shade
        'tertiary': '#ced4da', // Light Accent
        'quaternary': '#adb5bd', // Light Border
        'text-primary': '#212529', // Light Text
        'text-secondary': '#495057', // Light Subtext
        
        'dark-primary': '#302f3a', // Dark Background
        'dark-secondary': '#343a40', // Dark Shade
        'dark-tertiary': '#495057', // Dark Accent
        'dark-quaternary': '#6c757d', // Dark Border
        'dark-text-primary': '#f8f9fa', // Dark Text
        'dark-text-secondary': '#e9ecef', // Dark Subtext

        'interactive': '#9aa5b1', // Accent Hover
        'interactive-active': '#6c757d', // Accent Active
      },
      fontFamily: {
        sans: ['Avenir', 'Helvetica', 'Arial', 'sans-serif'],
      },
      boxShadow: {
        'custom-light': '0 2px 4px 0 rgba(0, 0, 0, 0.1)',
        'custom-dark': '0 2px 4px 0 rgba(0, 0, 0, 0.9)',
      },
    },
  },
  plugins: [],
};