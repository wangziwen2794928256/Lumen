import { locale } from '../../lib/locale';
import { useRef, useState } from 'react';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from './Field';
import type { Allocation, Decision, RoadmapSnapshot, RoadmapAction } from './model';
import { roadmapText as t } from './text';
function Allocations({ rows, data }: { rows: Allocation[]; data: RoadmapSnapshot }) {
  return rows.length ? (
    <ul className="roadmap-allocation-list">
      {rows.map((a, n) => (
        <li key={n}>
          <span>
            {a.date} · {data.tasks.find((task) => task.id === a.task_id)?.title ?? t.tasks}
          </span>
          <strong>
            {a.minutes} {t.minutes}
          </strong>
        </li>
      ))}
    </ul>
  ) : (
    <p className="muted">{t.noBlocks}</p>
  );
}
function DecisionCard({
  decision: d,
  data,
  busy,
  act,
}: {
  decision: Decision;
  data: RoadmapSnapshot;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const [editing, setEditing] = useState(false),
    [rows, setRows] = useState<Allocation[]>([]),
    [error, setError] = useState('');
  const decide = (operation: 'accept' | 'reject' | 'undo') =>
    void act({ type: 'decide', id: d.id, operation, allocations: null });
  return (
    <section className="roadmap-decision">
      <div className="roadmap-card-heading">
        <h3>{t.decisionStates[d.status]}</h3>
        <time>{new Date(d.created_at).toLocaleString(locale)}</time>
      </div>
      <p>{d.reason}</p>
      <div className="roadmap-budget-grid">
        {d.after.days.map((day) => (
          <div key={day.date}>
            <strong>{day.date}</strong>
            <p>
              {t.total} {day.capacity} · {t.fixedMinutes} {day.fixed_minutes}
            </p>
            <p>
              {t.free} {day.available} · {t.planned}{' '}
              {d.after.allocations
                .filter((a) => a.date === day.date)
                .reduce((n, a) => n + a.minutes, 0)}
            </p>
          </div>
        ))}
      </div>
      {d.after.warnings.map((w, n) => (
        <p key={n} className="muted">
          {w}
        </p>
      ))}
      <details>
        <summary>
          {t.reason} · {t.before} / {t.after}
        </summary>
        <h4>{t.before}</h4>
        <Allocations rows={d.before} data={data} />
        <h4>{t.after}</h4>
        <Allocations rows={d.after.allocations} data={data} />
        {d.after.unscheduled.length > 0 && (
          <>
            <h4>{t.unscheduled}</h4>
            <ul>
              {d.after.unscheduled.map(([id, n]) => (
                <li key={id}>
                  {data.tasks.find((task) => task.id === id)?.title ?? t.tasks} · {n} {t.minutes}
                </li>
              ))}
            </ul>
          </>
        )}
      </details>
      {d.status === 'proposed' && (
        <div className="roadmap-actions">
          <Button disabled={busy} onClick={() => decide('accept')}>
            {t.accept}
          </Button>
          <Button
            disabled={busy}
            variant="secondary"
            onClick={() => {
              setRows(d.after.allocations.map((a) => ({ ...a })));
              setEditing(true);
              setError('');
            }}
          >
            {t.modify}
          </Button>
          <Button disabled={busy} variant="ghost" onClick={() => decide('reject')}>
            {t.reject}
          </Button>
        </div>
      )}
      {['accepted', 'modified'].includes(d.status) && (
        <Button disabled={busy} variant="secondary" onClick={() => decide('undo')}>
          {t.undo}
        </Button>
      )}
      {editing && (
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            if (
              await act({
                type: 'decide',
                id: d.id,
                operation: 'modify',
                allocations: rows.filter((a) => a.minutes > 0),
              })
            )
              setEditing(false);
            else setError(t.error);
          }}
        >
          <fieldset disabled={busy} className="roadmap-fieldset">
            {rows.map((row, n) => (
              <div key={n} className="roadmap-modification-row">
                <FormGrid>
                  <Field
                    label={t.tasks}
                    value={row.task_id}
                    options={data.tasks.map((task) => ({ value: task.id, label: task.title }))}
                    onChange={(v) =>
                      setRows(rows.map((a, i) => (i === n ? { ...a, task_id: v } : a)))
                    }
                  />
                  <Field
                    label={t.allocationDate}
                    type="date"
                    value={row.date}
                    onChange={(v) => setRows(rows.map((a, i) => (i === n ? { ...a, date: v } : a)))}
                  />
                  <Field
                    label={t.allocationMinutes}
                    type="number"
                    min={0}
                    max={1440}
                    value={row.minutes}
                    onChange={(v) =>
                      setRows(rows.map((a, i) => (i === n ? { ...a, minutes: Number(v) } : a)))
                    }
                  />
                </FormGrid>
                <Button
                  type="button"
                  variant="ghost"
                  onClick={() => setRows(rows.filter((_, i) => i !== n))}
                >
                  {t.remove}
                </Button>
              </div>
            ))}
            <div className="roadmap-actions">
              <Button
                type="button"
                variant="secondary"
                disabled={!data.tasks.length}
                onClick={() =>
                  setRows([
                    ...rows,
                    { task_id: data.tasks[0].id, date: data.week_start, minutes: 25 },
                  ])
                }
              >
                {t.addAllocation}
              </Button>
              <Button type="submit">{t.apply}</Button>
              <Button type="button" variant="ghost" onClick={() => setEditing(false)}>
                {t.cancel}
              </Button>
            </div>
          </fieldset>
          {error && <p role="alert">{error}</p>}
        </form>
      )}
    </section>
  );
}
export function PlannerPanel({
  data,
  busy,
  act,
}: {
  data: RoadmapSnapshot;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const id = useRef('');
  return (
    <Card className="roadmap-card">
      <div className="roadmap-card-heading">
        <CardTitle>{t.planner}</CardTitle>
        <Button
          disabled={busy}
          onClick={async () => {
            if (!id.current) id.current = crypto.randomUUID();
            if (await act({ type: 'propose', id: id.current })) id.current = '';
          }}
        >
          {t.propose}
        </Button>
      </div>
      <p className="muted">{t.noProposal}</p>
      <Allocations rows={data.blocks} data={data} />
      <div className="roadmap-budget-grid">
        {data.budgets.map((day) => (
          <div key={day.date}>
            <strong>{day.date}</strong>
            <p>
              {t.total} {day.capacity} · {t.fixedMinutes} {day.fixed_minutes}
            </p>
            <p>
              {t.free} {day.available} · {t.planned}{' '}
              {data.blocks.filter((a) => a.date === day.date).reduce((n, a) => n + a.minutes, 0)}
            </p>
          </div>
        ))}
      </div>
      {data.budgets.some(
        (day) =>
          data.blocks.filter((a) => a.date === day.date).reduce((n, a) => n + a.minutes, 0) >
          day.available,
      ) && <p role="alert">{t.changedCapacity}</p>}
      <h3>{t.history}</h3>
      {data.decisions.map((d) => (
        <DecisionCard key={d.id} decision={d} data={data} busy={busy} act={act} />
      ))}
    </Card>
  );
}
