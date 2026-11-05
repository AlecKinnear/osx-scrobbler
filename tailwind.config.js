/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f5f8f6',
          100: '#e3ede8',
          200: '#c7dcd2',
          300: '#9bc4ad',
          400: '#6ba485',
          500: '#4a8a64',
          600: '#38704f',
          700: '#2d5940',
          800: '#264835',
          900: '#1f3b2c',
        },
        accent: {
          50: '#fef9f0',
          100: '#fdecd6',
          200: '#fad5ad',
          300: '#f6b673',
          400: '#f1923a',
          500: '#ee7617',
          600: '#dc5d0c',
          700: '#b8440b',
          800: '#933612',
          900: '#782e12',
        },
        neutral: {
          50: '#f8f8f7',
          100: '#efeeec',
          200: '#dbd9d5',
          300: '#bab6af',
          400: '#8e8a82',
          500: '#6b6760',
          600: '#524e49',
          700: '#3e3b37',
          800: '#2a2825',
          900: '#1a1917',
        },
        success: {
          500: '#22a06b',
          600: '#1b8557',
        },
        warning: {
          500: '#e0a800',
          600: '#c49000',
        },
        error: {
          500: '#d64545',
          600: '#b83838',
        },
      },
      fontFamily: {
        serif: ['Georgia', 'Cambria', 'Times New Roman', 'Times', 'serif'],
        sans: ['system-ui', '-apple-system', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
      },
      maxWidth: {
        'article': '720px',
      },
    },
  },
  plugins: [],
}
