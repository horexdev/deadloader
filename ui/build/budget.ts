// The first frame has to come within 800 ms (decision 120), and parsing script is most of that work in the WebView.
// The build fails when what the page loads before it can draw grows past its budget, so a screen that slips back into
// the main chunk is caught here and not in a slow start later.
import type { Plugin } from 'vite';

/** The parts of a build output the budget looks at. */
export type Chunks = Record<
  string,
  { type: 'chunk'; code: string; isEntry: boolean; imports: string[]; dynamicImports: string[] } | { type: 'asset' }
>;

/** The entry chunk and every chunk it imports statically (the browser fetches and parses them before it runs). */
export function firstLoad(bundle: Chunks): { bytes: number; files: string[] } {
  const seen = new Set<string>();
  const visit = (name: string): void => {
    const chunk = bundle[name];
    if (seen.has(name) || chunk?.type !== 'chunk') {
      return;
    }
    seen.add(name);
    chunk.imports.forEach(visit);
  };
  for (const [name, chunk] of Object.entries(bundle)) {
    if (chunk.type === 'chunk' && chunk.isEntry) {
      visit(name);
    }
  }
  const files = [...seen].sort();
  const encoder = new TextEncoder();
  const bytes = files.reduce((sum, name) => {
    const chunk = bundle[name];
    return sum + (chunk?.type === 'chunk' ? encoder.encode(chunk.code).length : 0);
  }, 0);
  return { bytes, files };
}

const kB = (bytes: number): string => `${(bytes / 1024).toFixed(1)} kB`;

/** Null when the first load fits in `maxBytes`, else what to tell the developer. */
export function overBudget(bundle: Chunks, maxBytes: number): string | null {
  const { bytes, files } = firstLoad(bundle);
  if (bytes <= maxBytes) {
    return null;
  }
  return (
    `the first load is ${kB(bytes)} (${files.join(', ')}), over its budget of ${kB(maxBytes)}: ` +
    'load the new code when it is needed (import())'
  );
}

/** Fails `vite build` when the first load is over `maxBytes`. */
export function firstLoadBudget(maxBytes: number): Plugin {
  return {
    name: 'deadloader-first-load-budget',
    apply: 'build',
    generateBundle(_, bundle) {
      const problem = overBudget(bundle as unknown as Chunks, maxBytes);
      if (problem) {
        this.error(problem);
      }
      const { bytes } = firstLoad(bundle as unknown as Chunks);
      this.info(`first load ${kB(bytes)} of ${kB(maxBytes)}`);
    },
  };
}
