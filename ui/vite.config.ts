import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [svelte()],
  build: {
    // Assets stay separate files: the CSP allows no data: URLs.
    assetsInlineLimit: 0,
    // WebView2 is evergreen Chromium.
    target: 'chrome120',
  },
  test: {
    include: ['src/**/*.test.ts'],
    // Tests read the design tokens as text; other CSS is not processed in tests.
    css: { include: [/theme\/tokens\.css/] },
  },
});
