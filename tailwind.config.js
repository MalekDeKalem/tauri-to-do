// tailwind.config.js
module.exports = {
  mode: "jit",
  content: ['./index.html', './src/**/*.{rs,html}'], // adjust for your Yew app
  theme: {
    extend: {
      keyframes: {
        'dash': {
          to: {
            backgroundPosition: '200% center',
          },
        },
      },
      animation: {
        'dash': 'dash 2s linear infinite',
      },
    },
  },
  plugins: [],
}
