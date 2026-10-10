import { lazy } from 'react';
export const ProfileRoute = lazy(() =>
  import('../../../pages/profile/ProfilePage').then((m) => ({ default: m.ProfilePage })),
);
