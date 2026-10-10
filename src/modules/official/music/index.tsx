import { lazy } from 'react';
export const MusicRoute = lazy(() =>
  import('../../../pages/music/MusicPage').then((m) => ({ default: m.MusicPage })),
);
