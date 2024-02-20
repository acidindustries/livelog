/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ["**/*.tera", "**/*.html", "**/*.js"],
  theme: {
    extend: {
      flexBasis: {
        "1/2-gap-4": "calc(0.5% - (1/2 * 1rem))"
      }
    },
  },
  plugins: [],
}

