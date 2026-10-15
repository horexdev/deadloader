/**
 * `pnpm dev:mock` only: the placeholder pictures the browser preview shows instead of the core's image protocol.
 * The page's address (sent as the request's referrer) can slow them down (`?scenario=pictures-slow`) or fail some
 * (`?scenario=pictures-failing`), so their loading and failed states can be seen; `&pictures=slow` or
 * `&pictures=failing` does the same next to another screen's scenario.
 */
export interface MockPicture {
  delayMs: number;
  status: number;
  body: string;
}

export function mockPicture(path: string, referer: string | undefined): MockPicture {
  const id = path.replace(/^\//, '');
  const seed = parseInt(id.slice(0, 8) || '0', 16) || 0;
  const scenario = scenarioOf(referer);
  if (scenario === 'pictures-failing' && seed % 3 === 0) {
    return { delayMs: 600, status: 404, body: '' };
  }
  const hue = parseInt(id.slice(0, 6) || '0', 16) % 360;
  return {
    delayMs: scenario === 'pictures-slow' ? 800 + (seed % 23) * 100 : 0,
    status: 200,
    body:
      `<svg xmlns="http://www.w3.org/2000/svg" width="530" height="298" viewBox="0 0 530 298">` +
      `<defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">` +
      `<stop offset="0" stop-color="hsl(${hue} 45% 38%)"/><stop offset="1" stop-color="hsl(${(hue + 60) % 360} 45% 22%)"/>` +
      `</linearGradient></defs><rect width="530" height="298" fill="url(#g)"/></svg>`,
  };
}

function scenarioOf(referer: string | undefined): string | null {
  try {
    const params = new URL(referer ?? '').searchParams;
    const pictures = params.get('pictures');
    return pictures ? `pictures-${pictures}` : params.get('scenario');
  } catch {
    return null;
  }
}
