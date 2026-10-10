import { lazy } from 'react';
export const VaultRoute = lazy(() =>
  import('../../../pages/vault/VaultPage').then((m) => ({ default: m.VaultPage })),
);
