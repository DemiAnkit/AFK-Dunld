import type { Config } from "tailwindcss";

export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        gray: {
          950: '#0a0a0f',
          900: '#1a1a24',
          800: '#252532',
          700: '#303040',
        }
      }
    },
  },
  plugins: [],
} satisfies Config;
