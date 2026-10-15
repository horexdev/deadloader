import { describe, expect, it } from 'vitest';
import { mockPicture } from './mock-pictures';

const page = (scenario?: string) => `http://localhost:5199/${scenario ? `?scenario=${scenario}` : ''}#/catalog`;

describe('the browser preview’s placeholder pictures', () => {
  it('answers at once with a coloured picture', () => {
    const reply = mockPicture('/0000012300000123', page());
    expect(reply.delayMs).toBe(0);
    expect(reply.status).toBe(200);
    expect(reply.body).toContain('<svg');
    expect(reply.body).toContain('hsl(');
  });

  it('takes a while in the slow-pictures scenario, so their loading state can be seen', () => {
    const delays = ['/00000001', '/00000002', '/00000003'].map((id) => mockPicture(id, page('pictures-slow')).delayMs);
    expect(delays.every((ms) => ms >= 800 && ms <= 3000)).toBe(true);
    expect(new Set(delays).size).toBeGreaterThan(1);
  });

  it('fails some pictures in the failing-pictures scenario', () => {
    const statuses = Array.from({ length: 9 }, (_, i) => mockPicture(`/0000000${i}`, page('pictures-failing')).status);
    expect(statuses).toContain(404);
    expect(statuses).toContain(200);
  });

  it('can be slowed down next to another screen’s scenario', () => {
    const referer = 'http://localhost:5199/?scenario=downloads&pictures=slow#/downloads';
    expect(mockPicture('/00000001', referer).delayMs).toBeGreaterThanOrEqual(800);
    expect(mockPicture('/00000001', 'not an address').delayMs).toBe(0);
  });
});
