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
