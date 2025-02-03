/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  // Disabled dark mode
  darkMode: "class",
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      ...colors,
      neon: {
        shade: {
          100: "#C1FF5C", // Primary color
          200: "#AEE653",
          300: "#9ACC4A",
          400: "#87B340",
          500: "#749937",
          600: "#61802E",
          700: "#4D6625",
          800: "#3A4C1C",
          900: "#273312",
          1000: "#141809",
        },
        tint: {
          100: "#C1FF5C", // Primary color
          200: "#C7FF6C",
          300: "#CDFF7D",
          400: "#D4FF8D",
          500: "#DAFF9D",
          600: "#E0FFAE",
          700: "#E6FFBE",
          800: "#ECFFCE",
          900: "#F3FFDE",
          1000: "#F3FFDE",
        },
      },
      cornflower: {
        shade: {
          100: "#9a5cff",
          200: "#8B53E6",
          300: "#7B4ACC",
          400: "#6C40B3",
          500: "#5C3799",
          600: "#4D2E80",
          700: "#3E2566",
          800: "#2E1C4C",
          900: "#1F1233",
          1000: "#1F1233",
        },
        tint: {
          100: "#9a5cff",
          200: "#A46CFF",
          300: "#AE7DFF",
          400: "#B88DFF",
          500: "#C29DFF",
          600: "#CDAEFF",
          700: "#D7BEFF",
          800: "#E1CEFF",
          900: "#EBDEFF",
          1000: "#F5EFFF",
        },
      },
      magenta: {
        shade: {
          100: "#FEC5CF",
          200: "#D453E6",
          300: "#BD4ACC",
          400: "#A540B3",
          500: "#8E3799",
          600: "#762E80",
          700: "#5E2566",
          800: "#471C4C",
          900: "#2F1233",
          1000: "#180919",
        },
        tint: {
          100: "#EC5CFF",
          200: "#EE6CFF",
          300: "#F07DFF",
          400: "#F28DFF",
          500: "#F49DFF",
          600: "#F6AEFF",
          700: "#F7BEFF",
          800: "#F9CEFF",
          900: "#FBDEFF",
          1000: "#FDEFFF",
        },
      },
      neutral: {
        50: "#f7f7f7",
        100: "#ffffff",
        200: "#DEE0E7",
        300: "#A3A8B2",
        400: "#777A81",
        500: "#565A5D",
        600: "#2F3033",
        700: "#212224",
        800: "#17181A",
        900: "#000000",
      },
      gray: {
        600: "#181818",
      },
    },
    fontSize: {
      xs: ["0.75rem", { lineHeight: "0.875rem" }], // legal
      sm: ["0.875rem", { lineHeight: "0.875rem" }], // eyebrow
      base: ["1rem", { lineHeight: "1.25rem" }], // body
      lg: ["1.125rem", { lineHeight: "1.375rem" }], // subcopy
      xl: ["1.25rem", { lineHeight: "1.50rem" }], // h6
      "2xl": ["1.5rem", { lineHeight: "1.75rem" }], // h5
      "3xl": ["2rem", { lineHeight: "2.25rem" }], // h4
      "4xl": ["2.5rem", { lineHeight: "2.75rem" }], // h3
      "5xl": ["3rem", { lineHeight: "3.25rem" }], // h2
      "6xl": ["4rem", { lineHeight: "4.25rem" }], // h1
      "7xl": ["4.5rem", { lineHeight: "1" }],
      "8xl": ["6rem", { lineHeight: "1" }],
      "9xl": ["8rem", { lineHeight: "1" }],
      interactive: ["1rem", { lineHeight: "1rem" }], // interactive
    },
    fontFamily: {
      ibm: ["IBM Plex mono", "monospace"],
      helvetica: ["Helvetica Neue", "sans-serif"],
    },
  },
  plugins: [],
};
