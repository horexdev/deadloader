import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [svelte()],
  // Component tests mount real components, so Svelte must resolve to its client build there.
  resolve: process.env.VITEST ? { conditions: ['browser'] } : undefined,
  build: {
    // Assets stay separate files: the CSP allows no data: URLs.
    assetsInlineLimit: 0,
    // WebView2 is evergreen Chromium.
    target: 'chrome120',
  },
  test: {
    include: ['src/**/*.test.ts'],
    environment: 'happy-dom',
    setupFiles: ['src/lib/test/setup.ts'],
    // Tests read the design stylesheets as text; other CSS is not processed in tests.
    css: { include: [/theme\/(tokens|components)\.css/] },
  },
});
