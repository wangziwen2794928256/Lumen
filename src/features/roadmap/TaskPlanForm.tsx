import { useState } from 'react';
import { Dialog } from '../../components/ui/dialog';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from './Field';
import { options } from './options';
import {
  isLive,
  taskOptionsSchema,
  type PlanningTask,
  type RoadmapSnapshot,
  type RoadmapAction,
} from './model';
import { roadmapText as t } from './text';
export function TaskPlanForm({
  task,
  data,
  busy,
  act,
}: {
  task: PlanningTask;
  data: RoadmapSnapshot;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const [open, setOpen] = useState(false),
    [draft, setDraft] = useState(task),
    [error, setError] = useState('');
  const live = data.entities.filter((e) => isLive(e, data.entities));
  return (
    <Dialog
      open={open}
      onOpenChange={(v) => {
        if (busy) return;
        if (v) {
          setDraft({ ...task, options: { ...task.options } });
          setError('');
        }
        setOpen(v);
      }}
      title={t.configure}
      description={task.title}
      trigger={
        <Button variant="secondary" disabled={busy}>
          {t.configure}
        </Button>
      }
    >
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          const result = taskOptionsSchema.safeParse(draft.options);
          if (!result.success) {
            setError(t.invalid);
            return;
          }
          if (
            await act({
              type: 'task_plan',
              id: task.id,
              revision: task.revision,
              journey_id: draft.journey_id,
              subject_id: draft.subject_id,
              quest_id: draft.quest_id,
              input: result.data,
            })
          )
            setOpen(false);
          else setError(t.error);
        }}
      >
        <fieldset className="roadmap-fieldset" disabled={busy}>
          {(['journey_id', 'subject_id', 'quest_id'] as const).map((key, n) => {
            const kind = (['journey', 'subject', 'quest'] as const)[n];
            return (
              <Field
                key={key}
                label={[t.journey, t.subject, t.quest][n]}
                value={draft[key] ?? ''}
                onChange={(v) =>
                  setDraft({
                    ...draft,
                    [key]: v || null,
                    ...(key === 'journey_id' ? { subject_id: null, quest_id: null } : {}),
                  })
                }
                options={[
                  { value: '', label: t.none },
                  ...live
                    .filter(
                      (e) =>
                        e.kind === kind &&
                        (kind === 'journey' || !e.journey_id || e.journey_id === draft.journey_id),
                    )
                    .map((e) => ({ value: e.id, label: e.title })),
                ]}
              />
            );
          })}
          <Field
            label={t.earliest}
            value={draft.options.earliest_start ?? ''}
            type="date"
            onChange={(v) =>
              setDraft({ ...draft, options: { ...draft.options, earliest_start: v || null } })
            }
          />
          <FormGrid>
            {(['minimum_session', 'preferred_session'] as const).map((key, n) => (
              <Field
                key={key}
                label={[t.minimumSession, t.preferredSession][n]}
                value={draft.options[key]}
                type="number"
                min={1}
                max={1440}
                onChange={(v) =>
                  setDraft({ ...draft, options: { ...draft.options, [key]: Number(v) } })
                }
              />
            ))}
          </FormGrid>
          <Field
            label={t.energy}
            value={draft.options.energy_level}
            options={options(t.energies)}
            onChange={(v) =>
              setDraft({
                ...draft,
                options: {
                  ...draft.options,
                  energy_level: v as PlanningTask['options']['energy_level'],
                },
              })
            }
          />
          {(['splittable', 'fixed_time'] as const).map((key, n) => (
            <label key={key} className="checkbox-label">
              <input
                type="checkbox"
                checked={draft.options[key]}
                onChange={(e) =>
                  setDraft({ ...draft, options: { ...draft.options, [key]: e.target.checked } })
                }
              />
              {[t.splittable, t.fixed][n]}
            </label>
          ))}
        </fieldset>
        {error && <p role="alert">{error}</p>}
        <Button disabled={busy} type="submit">
          {t.save}
        </Button>
      </form>
    </Dialog>
  );
}
