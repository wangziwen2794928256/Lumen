import { describe, it, expect } from 'vitest';
import { emptyProfile } from '../../src/features/profile/model';
import { safeUrl, validatePublic } from '../../website/site.js';
describe('public boundary', () => {
  it('defaults private and accepts only a public snapshot', () => {
    expect(emptyProfile.public).toBe(false);
    const data = {
      schema_version: '1.0',
      exported_at: '2026-10-08T00:00:00Z',
      profile: null,
      artifacts: [],
    };
    expect(validatePublic(data)).toEqual(data);
    expect(() => validatePublic({ ...data, private_notes: 'secret' })).toThrow();
    expect(() =>
      validatePublic({ ...data, profile: { ...emptyProfile, about: 'secret' } }),
    ).toThrow();
    expect(() => validatePublic({ ...data, schema_version: '2.0' })).toThrow();
  });
  it('rejects active schemes, credentials and local file URLs', () => {
    expect(safeUrl('https://example.org/portfolio')).toBe(true);
    for (const url of [
      'javascript:alert(1)',
      'file:///lumen.db',
      'data:text/html,secret',
      'https://user:password@example.org',
      'https://example.org/\n',
    ])
      expect(safeUrl(url)).toBe(false);
  });
});
