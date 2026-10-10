import { describe, it, expect } from 'vitest';
import { emptyArtifact, portfolio, tags } from '../../src/features/vault/model';
describe('Vault privacy and source fidelity', () => {
  it('only includes explicitly public live records without rewriting text', () => {
    const a = emptyArtifact();
    a.description = 'user verbatim';
    expect(portfolio([a])).toEqual([]);
    a.visibility = 'public';
    expect(portfolio([a])[0].description).toBe('user verbatim');
    expect(
      portfolio([
        { ...a, archived_at: 'now' },
        { ...a, deleted_at: 'now' },
      ]),
    ).toEqual([]);
  });
  it('normalizes user tags and defaults to private', () => {
    expect(tags('Rust, Python，Rust, ')).toEqual(['Rust', 'Python']);
    expect(emptyArtifact().visibility).toBe('private');
  });
});
