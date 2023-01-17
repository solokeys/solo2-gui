import daisyui from 'daisyui';
import typography from '@tailwindcss/typography';

export default {
  // daisy must go after typography, don't be alphabetically sorted
  plugins: [typography, daisyui],
    theme: {
    extend: {},
  },
  content: ["./index.html",'./src/**/*.{svelte,js,ts}'], // for unused CSS
  variants: {
    extend: {},
  },
}
