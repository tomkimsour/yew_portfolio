module.exports = {
  // purge: {
  //   enabled: process.env.NODE_ENV === "production",
  //   mode: "all",
  //   // source_code represents the rust (yew?) source code root
  // },
  content: [
    "src/**/*.rs",
    "index.html",
  ],
  darkMode: 'class', // 'media' or 'class'
  theme: {
    extend: {
      colors: {
        bkg: "var(--c-bg)",
        content: "var(--c-content)",
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
