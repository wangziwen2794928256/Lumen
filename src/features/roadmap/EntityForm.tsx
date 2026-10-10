import { useRef, useState } from 'react';
import { Dialog } from '../../components/ui/dialog';
import { Button } from '../../components/ui/button';
import {
  defaults,
  entitySchema,
  isLive,
  type Details,
  type Entity,
  type EntityInput,
  type Kind,
  type RoadmapAction,
} from './model';
import { roadmapText as t } from './text';
import { Field, FormGrid } from './Field';
import { options } from './options';
export function EntityForm({
  kind,
  entity,
  entities,
  busy,
  act,
}: {
  kind: Kind;
  entity?: Entity;
  entities: Entity[];
  busy: boolean;
  act: (action: RoadmapAction) => Promise<boolean>;
}) {
  const [open, setOpen] = useState(false),
    [draft, setDraft] = useState<EntityInput>({
      title: '',
      journey_id: null,
      data: { ...defaults },
    }),
    [error, setError] = useState('');
  const id = useRef('');
  const set = (key: keyof Details, value: Details[keyof Details]) =>
    setDraft({ ...draft, data: { ...draft.data, [key]: value } });
  const parents = entities.filter(
    (e) =>
      e.id !== entity?.id &&
      isLive(e, entities) &&
      e.kind === (kind === 'subject' ? 'subject' : 'journey'),
  );
  const journeys = entities.filter((e) => e.kind === 'journey' && isLive(e, entities));
  return (
    <Dialog
      open={open}
      onOpenChange={(value) => {
        if (busy) return;
        if (value) {
          setDraft(
            entity
              ? { title: entity.title, journey_id: entity.journey_id, data: { ...entity.data } }
              : {
                  title: '',
                  journey_id: kind === 'phase' ? (journeys[0]?.id ?? null) : null,
                  data: { ...defaults },
                },
          );
          id.current = crypto.randomUUID();
          setError('');
        }
        setOpen(value);
      }}
      trigger={
        <Button variant={entity ? 'secondary' : 'default'} disabled={busy}>
          {entity ? t.edit : t.add + t.kinds[kind]}
        </Button>
      }
      title={(entity ? t.edit : t.add) + t.kinds[kind]}
      description={t.formHint}
    >
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          const result = entitySchema.safeParse(draft);
          if (!result.success || (kind === 'phase' && !draft.journey_id)) {
            setError(t.invalid);
            return;
          }
          if (
            await act({
              type: 'save',
              id: entity?.id ?? id.current,
              kind,
              revision: entity?.revision ?? null,
              input: result.data,
            })
          )
            setOpen(false);
          else setError(t.error);
        }}
      >
        <fieldset disabled={busy} className="roadmap-fieldset">
          <Field
            label={t.titleLabel}
            value={draft.title}
            onChange={(title) => setDraft({ ...draft, title })}
            required
          />
          <Field
            label={t.description}
            value={draft.data.description}
            onChange={(v) => set('description', v)}
            multiline
          />
          {kind !== 'journey' && (
            <Field
              label={t.journey}
              value={draft.journey_id ?? ''}
              onChange={(v) => setDraft({ ...draft, journey_id: v || null })}
              options={[
                { value: '', label: t.none },
                ...journeys.map((e) => ({ value: e.id, label: e.title })),
              ]}
            />
          )}
          <FormGrid>
            <Field
              label={t.start}
              type="date"
              value={draft.data.start_date ?? ''}
              onChange={(v) => set('start_date', v || null)}
            />
            <Field
              label={t.deadline}
              type="date"
              value={draft.data.target_date ?? ''}
              onChange={(v) => set('target_date', v || null)}
            />
          </FormGrid>
          <FormGrid>
            {(['minimum_minutes', 'preferred_minutes', 'maximum_minutes'] as const).map(
              (key, n) => (
                <Field
                  key={key}
                  label={[t.minimum, t.preferred, t.maximum][n]}
                  type="number"
                  min={0}
                  max={10080}
                  value={draft.data[key]}
                  onChange={(v) => set(key, Number(v))}
                />
              ),
            )}
          </FormGrid>
          <FormGrid>
            <Field
              label={t.category}
              value={draft.data.category}
              onChange={(v) => set('category', v)}
            />
            <Field
              label={t.status}
              value={draft.data.status}
              onChange={(v) => set('status', v)}
              options={options(t.statusOptions)}
            />
            <Field
              label={t.priority}
              type="number"
              min={0}
              max={3}
              value={draft.data.priority}
              onChange={(v) => set('priority', Number(v))}
            />
          </FormGrid>
          <details>
            <summary>
              {t.progress} / {t.preparation}
            </summary>
            <FormGrid>
              <Field
                label={t.progressMode}
                value={draft.data.progress_mode}
                options={options(
                  kind === 'phase' ? { manual: t.progressModes.manual } : t.progressModes,
                )}
                onChange={(v) => set('progress_mode', v)}
              />
              <Field
                label={t.progress}
                type="number"
                min={0}
                max={100}
                value={draft.data.progress}
                onChange={(v) => set('progress', Number(v))}
              />
              <Field
                label={t.position}
                type="number"
                min={0}
                max={10000}
                value={draft.data.position}
                onChange={(v) => set('position', Number(v))}
              />
            </FormGrid>
            {(kind === 'journey' || kind === 'subject') && (
              <Field
                label={t.parent}
                value={draft.data.parent_id ?? ''}
                onChange={(v) => set('parent_id', v || null)}
                options={[
                  { value: '', label: t.none },
                  ...parents.map((e) => ({ value: e.id, label: e.title })),
                ]}
              />
            )}
            <Field
              label={t.preparation}
              value={draft.data.preparation_phase}
              onChange={(v) => set('preparation_phase', v)}
            />
            <FormGrid>
              <Field
                label={t.color}
                value={draft.data.color}
                options={options(t.colors)}
                onChange={(v) => set('color', v)}
              />
              <Field label={t.icon} value={draft.data.icon} onChange={(v) => set('icon', v)} />
            </FormGrid>
            <Field
              label={t.target}
              type="number"
              min={0}
              max={750}
              value={draft.data.planning_target ?? ''}
              onChange={(v) => set('planning_target', v === '' ? null : Number(v))}
            />
            <p className="muted">{t.targetHint}</p>
          </details>
        </fieldset>
        {error && <p role="alert">{error}</p>}
        <Button type="submit" disabled={busy}>
          {t.save}
        </Button>
      </form>
    </Dialog>
  );
}
