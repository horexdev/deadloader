import { svelte } from '@sveltejs/vite-plugin-svelte';
import { fileURLToPath } from 'node:url';
import type { Plugin } from 'vite';
import { defineConfig } from 'vitest/config';
import { firstLoadBudget } from './build/budget.ts';

/**
 * `pnpm dev:mock` only: catalog pictures as coloured placeholders, so screens can be previewed in a browser without
 * the core's image protocol.
 */
function mockPictures(): Plugin {
  return {
    name: 'deadloader-mock-pictures',
    configureServer(server) {
      server.middlewares.use('/__mock/img', (req, res) => {
        const id = (req.url ?? '').replace(/^\//, '');
        const hue = parseInt(id.slice(0, 6) || '0', 16) % 360;
        res.setHeader('Content-Type', 'image/svg+xml');
        res.end(
          `<svg xmlns="http://www.w3.org/2000/svg" width="530" height="298" viewBox="0 0 530 298">` +
            `<defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">` +
            `<stop offset="0" stop-color="hsl(${hue} 45% 38%)"/><stop offset="1" stop-color="hsl(${(hue + 60) % 360} 45% 22%)"/>` +
            `</linearGradient></defs><rect width="530" height="298" fill="url(#g)"/></svg>`,
        );
      });
    },
  };
}

export default defineConfig(({ mode }) => ({
  // What the page loads before its first frame (decision 120); screens and the wizard load when shown.
  plugins: [svelte(), firstLoadBudget(272 * 1024), ...(mode === 'mock' ? [mockPictures()] : [])],
  resolve: process.env.VITEST
    ? // Component tests mount real components, so Svelte must resolve to its client build there.
      { conditions: ['browser'] }
    : mode === 'mock'
      ? // The browser preview answers the core's commands from invented data (src/mock).
        { alias: { '@tauri-apps/api/core': fileURLToPath(new URL('./src/mock/core.ts', import.meta.url)) } }
      : undefined,
  // Screens and the wizard load lazily; the dev server scans them at start, so their first visit doesn't make it
  // re-bundle the dependencies and reload the page.
  optimizeDeps: { entries: ['index.html', 'src/screens/*/Screen.svelte', 'src/wizard/Wizard.svelte'] },
  // The i18n tests read the Rust sources (message ids, reasons, language codes) to stay in step with the core.
  server: process.env.VITEST ? { fs: { allow: ['.', '../crates'] } } : undefined,
  build: {
    // Assets stay separate files: the CSP allows no data: URLs.
    assetsInlineLimit: 0,
    // WebView2 is evergreen Chromium.
    target: 'chrome120',
  },
  test: {
    include: ['src/**/*.test.ts', 'build/**/*.test.ts'],
    environment: 'happy-dom',
    setupFiles: ['src/lib/test/setup.ts'],
    // Tests read the design stylesheets and the font stacks per script as text; other CSS is not processed in tests.
    css: { include: [/theme\/(tokens|components)\.css/, /i18n\/fonts\.css/] },
  },
}));
