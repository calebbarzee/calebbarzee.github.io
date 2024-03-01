/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  darkMode: 'media', // or 'class'
  theme: {
    extend: {
      colors: {
        // Existing colors remain unchanged
        'primary': '#eae0d5',
        'secondary': '#e4e6eb',
        // Adding more shades for detailed ASCII art
        'lightest': '#f8f9fa',
        'lighter': '#f1f3f5',
        'light-medium': '#dee2e6',
        'medium': '#ced4da',
        'dark-medium': '#adb5bd',
        'darker': '#6c757d',
        'darkest': '#495057',
        // Dark theme colors remain unchanged
        'dark-primary': '#302f3a',
        'dark-secondary': '#343a40',
        'dark-tertiary': '#495057',
        'dark-quaternary': '#6c757d',
        'dark-text-primary': '#f8f9fa',
        'dark-text-secondary': '#e9ecef',
        'interactive': '#9aa5b1',
        'interactive-active': '#6c757d',
      },
      fontFamily: {
        "monospace": ['"Fira Code"', '"Roboto Mono"', 'monospace'],
      },
      fontSize: {
        // Refining font sizes for better control in ASCII art
        xs: '0.75rem', // Adjusted for better visibility
        sm: '0.875rem',
        base: '1rem',
        lg: '1.125rem',
        xl: '1.25rem',
        '2xl': '1.5rem',
        '3xl': '1.875rem',
        '4xl': '2.25rem',
        '5xl': '3rem',
        '6xl': '4rem',
      },
      lineHeight: {
        // Custom line heights for ASCII consistency
        tight: 1.1,
        snug: 1.2,
        normal: 1.5,
        relaxed: 1.625,
        loose: 2,
      },
      boxShadow: {
        // Existing shadows remain unchanged
        'custom-light': '0 2px 4px 0 rgba(0, 0, 0, 0.1)',
        'custom-dark': '0 2px 4px 0 rgba(0, 0, 0, 0.9)',
      },
    },
  },
  plugins: [],
};
