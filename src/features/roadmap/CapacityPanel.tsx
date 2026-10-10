import { useState } from 'react';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from './Field';
import { options } from './options';
import type { CapacityDay, Preferences, RoadmapAction, RoadmapSnapshot } from './model';
import { roadmapText as t } from './text';
import { addDays } from '../calendar/model';
function PreferencesForm({
  initial,
  busy,
  act,
}: {
  initial: Preferences;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const [draft, setDraft] = useState(initial),
    [error, setError] = useState('');
  return (
    <form
      onSubmit={async (e) => {
        e.preventDefault();
        if (!(await act({ type: 'preferences', input: draft }))) setError(t.error);
      }}
    >
      <fieldset disabled={busy} className="roadmap-fieldset">
        <FormGrid>
          {(
            ['weekday_minutes', 'weekend_minutes', 'side_quest_max', 'planning_target'] as const
          ).map((key, n) => (
            <Field
              key={key}
              label={[t.weekday, t.weekend, t.sideMaximum, t.target][n]}
              value={draft[key]}
              type="number"
              min={0}
              max={n < 2 ? 1440 : n === 2 ? 10080 : 750}
              onChange={(v) => setDraft({ ...draft, [key]: Number(v) })}
            />
          ))}
        </FormGrid>
        <p className="muted">{t.targetHint}</p>
        <Button type="submit" disabled={busy}>
          {t.save}
        </Button>
      </fieldset>
      {error && <p role="alert">{error}</p>}
    </form>
  );
}
function DayForm({
  initial,
  busy,
  act,
}: {
  initial: CapacityDay;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const [draft, setDraft] = useState(initial),
    [error, setError] = useState('');
  return (
    <form
      className="roadmap-day"
      onSubmit={async (e) => {
        e.preventDefault();
        if (await act({ type: 'capacity', input: draft })) setError('');
        else setError(t.error);
      }}
    >
      <h3>{initial.date}</h3>
      <fieldset className="roadmap-fieldset" disabled={busy}>
        <Field
          label={t.dayMinutes}
          value={draft.minutes}
          type="number"
          min={0}
          max={1440}
          onChange={(v) => setDraft({ ...draft, minutes: Number(v) })}
        />
        <Field
          label={t.dayStatus}
          value={draft.status}
          options={options(t.dayStatuses)}
          onChange={(v) => setDraft({ ...draft, status: v as CapacityDay['status'] })}
        />
        <Button type="submit" variant="secondary" disabled={busy}>
          {t.save}
        </Button>
      </fieldset>
      {error && <p role="alert">{error}</p>}
    </form>
  );
}
export function CapacityPanel({
  data,
  busy,
  act,
  preferencesOnly = false,
}: {
  data: RoadmapSnapshot;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
  preferencesOnly?: boolean;
}) {
  return (
    <>
      <Card className="roadmap-card">
        <CardTitle>{t.preferences}</CardTitle>
        <PreferencesForm
          key={data.preferences.revision}
          initial={data.preferences}
          busy={busy}
          act={act}
        />
      </Card>
      {!preferencesOnly && (
        <Card className="roadmap-card">
          <CardTitle>{t.weekCapacity}</CardTitle>
          <p className="muted">{t.capacityHint}</p>
          <div className="roadmap-days">
            {Array.from({ length: 7 }, (_, n) => {
              const date = addDays(data.week_start, n),
                saved = data.days.find((d) => d.date === date);
              return (
                <DayForm
                  key={
                    date +
                    ':' +
                    (saved
                      ? saved.minutes + saved.status
                      : n < 5
                        ? data.preferences.weekday_minutes
                        : data.preferences.weekend_minutes)
                  }
                  initial={
                    saved ?? {
                      date,
                      minutes:
                        n < 5 ? data.preferences.weekday_minutes : data.preferences.weekend_minutes,
                      status: 'normal',
                    }
                  }
                  busy={busy}
                  act={act}
                />
              );
            })}
          </div>
        </Card>
      )}
    </>
  );
}
