/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ['./templates/**/*.html'],
  },
  theme: {
    extend: {
      minWidth: ({ theme }) => ({ ...theme('space') }),
    },
  },
  plugins: [],
}
