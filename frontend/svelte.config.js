import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: undefined,
			precompress: false,
			strict: true
		}),
		paths: {
			// デプロイ時のみ BASE_PATH を渡す (deploy.yml で設定)。
			// `vite build` は NODE_ENV=production を自動で立てるため、
			// NODE_ENV ベースの分岐だと CI のテスト build まで base 付きになり
			// preview / Playwright が 404 になっていた。
			base: process.env.BASE_PATH ?? ''
		}
	}
};

export default config;
