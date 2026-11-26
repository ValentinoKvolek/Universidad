/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        unlp: {
          primary: '#1e293b',   // Slate 800
          accent: '#3b82f6',    // Cobalt Blue
          academic: '#0f172a',  // Deep Slate
          gold: '#f59e0b',      // Amber
          emerald: '#10b981',   // Pascal Success
          crimson: '#ef4444',   // Memory Leak/Error
          card: '#1e293b',
          border: '#334155'
        }
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', 'Fira Code', 'Menlo', 'monospace'],
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
