import { lazy } from 'react';
import { FortuneCard } from '../../../features/fortune/FortuneCard';
export const FortuneRoute = lazy(() =>
  import('../../../pages/fortune/FortunePage').then((m) => ({ default: m.FortunePage })),
);
// Bundled official contribution, with no third-party code loading.
// eslint-disable-next-line react-refresh/only-export-components
export const fortuneWidgets = [
  {
    id: 'lumen.fortune.daily',
    moduleId: 'lumen.fortune',
    slot: 'today.sidebar',
    order: 20,
    Component: FortuneCard,
  },
] as const;
