/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      "./src/**/*.rs",
      "./assets/**/*.css",
      "./templates/**/*.j2",
  ],
  theme: {
    extend: {},
  },
  plugins: [
      require('tailwindcss-iconify').default(),
  ],
}

