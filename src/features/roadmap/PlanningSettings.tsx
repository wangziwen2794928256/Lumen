import { useRoadmap } from './useRoadmap';
import { weekStart } from './model';
import { CapacityPanel } from './CapacityPanel';
import { roadmapText as t } from './text';
export function PlanningSettings() {
  const { data, error, busy, browser, act } = useRoadmap(weekStart());
  return (
    <div className="roadmap-settings">
      {browser && <p className="muted">{t.browser}</p>}
      {error && <p role="alert">{error}</p>}
      {data && <CapacityPanel data={data} busy={busy} act={act} preferencesOnly />}
    </div>
  );
}
