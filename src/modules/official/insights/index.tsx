import { lazy } from 'react';
import { Memories } from '../../../features/insights/Memories';
export const InsightsRoute = lazy(() =>
  import('../../../pages/insights/InsightsPage').then((m) => ({ default: m.InsightsPage })),
);
// Static bundled contribution descriptor; never an untrusted dynamic loader.
// eslint-disable-next-line react-refresh/only-export-components
export const insightsWidgets = [
  {
    id: 'lumen.insights.on-this-day',
    moduleId: 'lumen.insights',
    slot: 'today.after-summary',
    order: 10,
    Component: Memories,
  },
] as const;
