import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
// import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

import adapter from "@sveltejs/adapter-static"; // <-- adapter-static replaces adapter-auto
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [preprocess(), vitePreprocess({})],

  kit: {
    // some settings for adapter-static:
    adapter: adapter({
      pages: "build",
      assets: "build",
      fallback: "index.html",
    }),
  },
};

export default config;
