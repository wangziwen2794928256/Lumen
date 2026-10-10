import { it, expect } from 'vitest';
import { musicCloud, localDateTime } from '../../src/features/music/model';
it('music clouds aggregate separately by song identity artist and album', () => {
  const rows = [
    { title: 'Same', artist: 'A', album: 'Collection', count: 2, seconds: 60 },
    { title: 'Same', artist: 'B', album: 'Collection', count: 1, seconds: 40 },
  ];
  expect(musicCloud(rows, 'title')).toHaveLength(2);
  expect(musicCloud(rows, 'album')).toEqual([{ label: 'Collection', count: 3, seconds: 100 }]);
  expect(musicCloud(rows, 'artist')[0].count).toBe(2);
});
it('local editing time retains the instant in the current timezone', () => {
  const stamp = '2026-10-04T04:00:00Z';
  expect(new Date(localDateTime(stamp)).toISOString()).toBe(new Date(stamp).toISOString());
});
