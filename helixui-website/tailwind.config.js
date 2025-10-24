/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: 'class',
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
    "../helixui-lib/src/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0fdf4',
          100: '#dcfce7',
          200: '#bbf7d0',
          300: '#86efac',
          400: '#4ade80',
          500: '#22c55e',
          600: '#16a34a',
          700: '#15803d',
          800: '#166534',
          900: '#14532d',
        },
      },
      keyframes: {
        'tooltip-fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'alert-animate-in': {
          '0%': { 
            opacity: '0',
            transform: 'scale(0.95) translateY(-2px)'
          },
          '100%': { 
            opacity: '1',
            transform: 'scale(1) translateY(0)'
          },
        },
        'alert-animate-out': {
          '0%': { 
            opacity: '1',
            transform: 'scale(1) translateY(0)'
          },
          '100%': { 
            opacity: '0',
            transform: 'scale(0.95) translateY(-2px)'
          },
        },
        'dialog-overlay-show': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'dialog-overlay-hide': {
          '0%': { opacity: '1' },
          '100%': { opacity: '0' },
        },
        'dialog-content-show': {
          '0%': {
            opacity: '0',
            transform: 'scale(0.95) translate(-50%, -50%)'
          },
          '100%': {
            opacity: '1',
            transform: 'scale(1) translate(-50%, -50%)'
          },
        },
        'dialog-content-hide': {
          '0%': {
            opacity: '1',
            transform: 'scale(1) translate(-50%, -50%)'
          },
          '100%': {
            opacity: '0',
            transform: 'scale(0.95) translate(-50%, -50%)'
          },
        },
      },
      animation: {
        'tooltip-fade-in': 'tooltip-fade-in 0.2s ease-in-out',
        'alert-animate-in': 'alert-animate-in 150ms ease-out forwards',
        'alert-animate-out': 'alert-animate-out 150ms ease-in forwards',
        'dialog-overlay-show': 'dialog-overlay-show 150ms ease-out forwards',
        'dialog-overlay-hide': 'dialog-overlay-hide 150ms ease-in forwards',
        'dialog-content-show': 'dialog-content-show 150ms ease-out forwards',
        'dialog-content-hide': 'dialog-content-hide 150ms ease-in forwards',
      },
    },
  },
  plugins: [
    function({ addComponents }) {
      addComponents({
        '.toast-container': {
          '&:hover .toast': {
            'margin-top': '0 !important',
            'opacity': '1 !important',
            'transform': 'scale(1) !important',
            'filter': 'brightness(1) !important',
          },
        },
      })
    }
  ],
};
