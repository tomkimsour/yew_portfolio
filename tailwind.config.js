module.exports = {
  // purge: {
  //   enabled: process.env.NODE_ENV === "production",
  //   mode: "all",
  //   // source_code represents the rust (yew?) source code root
  // },
  content: [
    "src/**/*.rs",
    "index.html",
    // "./input/tailwind.css",
  ],
  darkMode: 'class', // 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
