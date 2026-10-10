import { useRef, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Input } from '../../components/ui/input';
import { roadmapText as t } from '../../features/roadmap/text';
import { weekStart, isLive, type Entity, type Kind } from '../../features/roadmap/model';
import { useRoadmap } from '../../features/roadmap/useRoadmap';
import { EntityForm } from '../../features/roadmap/EntityForm';
import { TaskPlanForm } from '../../features/roadmap/TaskPlanForm';
import { CapacityPanel } from '../../features/roadmap/CapacityPanel';
import { PlannerPanel } from '../../features/roadmap/PlannerPanel';
import { RulesPanel } from '../../features/roadmap/RulesPanel';
import { TaskForm } from '../../features/today/TaskForm';
import { useToday } from '../../features/today/useToday';
import { addDays } from '../../features/calendar/model';
export function RoadmapPage() {
  const [searchParams] = useSearchParams();
  const [week, setWeek] = useState(weekStart),
    { data, loading, browser, error, busy, act, refresh } = useRoadmap(week),
    today = useToday();
  const templateId = useRef('');
  const entityCard = (e: Entity) => (
    <article
      key={e.id}
      className="roadmap-entity"
      data-deep-link-selected={e.id === searchParams.get('journey') || undefined}
    >
      <div>
        <p className="eyebrow">
          {t.kinds[e.kind]} · {t.statusOptions[e.data.status as keyof typeof t.statusOptions]}
        </p>
        <h3>{e.title}</h3>
        {e.journey_id && (
          <p className="muted">{data?.entities.find((p) => p.id === e.journey_id)?.title}</p>
        )}
        <p className="preserve-lines">{e.data.description}</p>
        <p>
          {e.data.start_date} {e.data.target_date ? '→ ' + e.data.target_date : ''}
        </p>
        <p>
          {t.minimum} {e.data.minimum_minutes} · {t.preferred} {e.data.preferred_minutes} ·{' '}
          {t.maximum} {e.data.maximum_minutes}
        </p>
        <progress value={e.data.progress} max={100} aria-label={e.title + ' · ' + t.progress} />
        <span> {e.data.progress}%</span>
        {e.data.planning_target !== null && (
          <p>
            {t.target} {e.data.planning_target} · {t.targetHint}
          </p>
        )}
      </div>
      <div className="roadmap-actions">
        <EntityForm
          kind={e.kind}
          entity={e}
          entities={data?.entities ?? []}
          busy={busy}
          act={act}
        />
        <Button
          disabled={busy}
          variant="ghost"
          onClick={() =>
            void act({
              type: 'lifecycle',
              id: e.id,
              kind: e.kind,
              revision: e.revision,
              operation: 'archive',
            })
          }
        >
          {t.archive}
        </Button>
        <Button
          disabled={busy}
          variant="ghost"
          onClick={() =>
            void act({
              type: 'lifecycle',
              id: e.id,
              kind: e.kind,
              revision: e.revision,
              operation: 'trash',
            })
          }
        >
          {t.trash}
        </Button>
      </div>
    </article>
  );
  return (
    <>
      <div className="page-heading">
        <div>
          <p className="eyebrow">ROADMAP / {t.kinds.journey}</p>
          <h1>{t.title}</h1>
          <p className="intro">{t.intro}</p>
        </div>
        <Button variant="secondary" onClick={() => void refresh()} disabled={busy}>
          {t.refresh}
        </Button>
      </div>
      <div className="roadmap-week">
        <label htmlFor="planning-week">{t.week}</label>
        <Input
          id="planning-week"
          type="date"
          value={week}
          disabled={busy}
          onChange={(e) => {
            if (e.target.value) setWeek(weekStart(e.target.value));
          }}
        />
        <Button variant="secondary" disabled={busy} onClick={() => setWeek(addDays(week, -7))}>
          {t.previous}
        </Button>
        <Button variant="secondary" disabled={busy} onClick={() => setWeek(weekStart())}>
          {t.thisWeek}
        </Button>
        <Button variant="secondary" disabled={busy} onClick={() => setWeek(addDays(week, 7))}>
          {t.next}
        </Button>
      </div>
      {error && <p role="alert">{error}</p>}
      {loading && <p role="status">{t.loading}</p>}
      {browser && <p role="status">{t.browser}</p>}
      {data && (
        <>
          <Card className="roadmap-card">
            <div className="roadmap-card-heading">
              <CardTitle>{t.kinds.journey}</CardTitle>
              <div className="roadmap-actions">
                <EntityForm kind="journey" entities={data.entities} busy={busy} act={act} />
                <Button
                  variant="secondary"
                  disabled={busy}
                  onClick={async () => {
                    if (!templateId.current) templateId.current = crypto.randomUUID();
                    if (await act({ type: 'template', id: templateId.current }))
                      templateId.current = '';
                  }}
                >
                  {t.template}
                </Button>
              </div>
            </div>
            <p className="muted">{t.templateHint}</p>
            {!data.entities.some((e) => e.kind === 'journey' && isLive(e, data.entities)) && (
              <p>{t.empty}</p>
            )}
            {data.entities
              .filter((e) => e.kind === 'journey' && isLive(e, data.entities))
              .map(entityCard)}
          </Card>
          {(['phase', 'subject', 'quest'] as Kind[]).map((kind) => (
            <Card key={kind} className="roadmap-card">
              <div className="roadmap-card-heading">
                <CardTitle>{t.kinds[kind]}</CardTitle>
                <EntityForm kind={kind} entities={data.entities} busy={busy} act={act} />
              </div>
              <div className="roadmap-entity-grid">
                {data.entities
                  .filter((e) => e.kind === kind && isLive(e, data.entities))
                  .sort((a, b) => a.data.position - b.data.position)
                  .map(entityCard)}
              </div>
            </Card>
          ))}
          <Card className="roadmap-card">
            <CardTitle>{t.tasks}</CardTitle>
            <TaskForm
              disabled={browser}
              busy={busy || today.busy}
              act={async (a) => {
                const ok = await today.act(a);
                if (ok) await refresh();
                return ok;
              }}
            />
            {!data.tasks.length && <p className="muted">{t.taskEmpty}</p>}
            {data.tasks.map((task) => (
              <article key={task.id} className="roadmap-task-row">
                <div>
                  <h3>{task.title}</h3>
                  <p>
                    {t.remaining} {task.remaining_minutes} {t.minutes} · {task.deadline ?? t.none}
                  </p>
                  <p className="muted">
                    {data.entities.find((e) => e.id === task.journey_id)?.title}{' '}
                    {data.entities.find((e) => e.id === task.subject_id)?.title}{' '}
                    {data.entities.find((e) => e.id === task.quest_id)?.title}
                  </p>
                </div>
                <TaskPlanForm task={task} data={data} busy={busy} act={act} />
              </article>
            ))}
          </Card>
          <CapacityPanel key={week} data={data} busy={busy} act={act} />
          <PlannerPanel key={week + '-planner'} data={data} busy={busy} act={act} />
          <RulesPanel data={data} busy={busy} act={act} />
          <Card className="roadmap-card">
            <details>
              <summary>{t.recycle}</summary>
              {data.entities
                .filter((e) => e.deleted_at || e.archived_at)
                .map((e) => (
                  <div key={e.id} className="roadmap-task-row">
                    <h3>{e.title}</h3>
                    <div className="roadmap-actions">
                      {e.deleted_at && (
                        <Button
                          disabled={busy}
                          variant="secondary"
                          onClick={() =>
                            void act({
                              type: 'lifecycle',
                              id: e.id,
                              kind: e.kind,
                              revision: e.revision,
                              operation: 'restore',
                            })
                          }
                        >
                          {t.restore}
                        </Button>
                      )}
                      {e.archived_at && (
                        <Button
                          disabled={busy}
                          variant="secondary"
                          onClick={() =>
                            void act({
                              type: 'lifecycle',
                              id: e.id,
                              kind: e.kind,
                              revision: e.revision,
                              operation: 'unarchive',
                            })
                          }
                        >
                          {t.unarchive}
                        </Button>
                      )}
                    </div>
                  </div>
                ))}
            </details>
          </Card>
        </>
      )}
    </>
  );
}
