/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0f5fb',
          100: '#dde8f5',
          200: '#bcd0ec',
          300: '#8bafd9',
          400: '#5a8cc4',
          500: '#3a6fa8',
          600: '#1f4579',
          700: '#1a3a66',
          800: '#163153',
          900: '#122844',
        },
        accent: {
          50: '#e6faf8',
          100: '#c4f2ee',
          200: '#94e7e0',
          300: '#5dd6cc',
          400: '#2cc4ba',
          500: '#00ae9f',
          600: '#009085',
          700: '#00736b',
          800: '#005a54',
          900: '#004640',
        },
        ink: {
          50: '#f6f6f6',
          100: '#ececec',
          200: '#d5d5d5',
          300: '#b0b0b0',
          400: '#7a7a7a',
          500: '#5a5a5a',
          600: '#3e3e3e',
          700: '#2d2d2d',
          800: '#232323',
          900: '#1a1a1a',
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
