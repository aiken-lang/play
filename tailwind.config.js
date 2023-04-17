/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Source Sans Pro"],
      },
    },
  },
  plugins: [],
}

