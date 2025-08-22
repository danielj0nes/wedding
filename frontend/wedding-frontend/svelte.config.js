import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// Using adapter-static to produce a static site suitable for serving from axum
		// enable `fallback: 'index.html'` so client-side dynamic routes work with a single-page fallback
		adapter: adapter({ pages: 'build', assets: 'build', fallback: 'index.html' })
	}
};

export default config;
