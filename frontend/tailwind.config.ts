import type { Config } from "tailwindcss";

export default {
  content: ["./src/lib/**/*.{js,ts,jsx,tsx,mdx}", "./src/app/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-noto-sans-tc)"],
        mono: ["var(--font-hack-nerd-mono)", "var(--font-noto-sans-mono)"],
      },
      colors: {
        "true-gray": {
          "50": "#fafafa",
          "100": "#f5f5f5",
          "200": "#e5e5e5",
          "300": "#d4d4d4",
          "400": "#a3a3a3",
          "500": "#737373",
          "600": "#525252",
          "700": "#404040",
          "800": "#262626",
          "900": "#171717",
        },
      },
    },
  },
  plugins: [],
} satisfies Config;
