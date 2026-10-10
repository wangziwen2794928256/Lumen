import { useRef, useState } from 'react';
import { Dialog } from '../../components/ui/dialog';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from './Field';
import type { RoadmapSnapshot, RoadmapAction, RuleInput } from './model';
import { roadmapText as t } from './text';
const initial = (): RuleInput => ({
  journey_id: null,
  school: '',
  college: '',
  major: '',
  year: new Date().getFullYear() + 1,
  written_exam: '',
  interview: '',
  source_url: '',
  body: '',
  notes: '',
});
export function RulesPanel({
  data,
  busy,
  act,
}: {
  data: RoadmapSnapshot;
  busy: boolean;
  act: (a: RoadmapAction) => Promise<boolean>;
}) {
  const [open, setOpen] = useState(false),
    [draft, setDraft] = useState(initial),
    [error, setError] = useState('');
  const id = useRef('');
  return (
    <Card className="roadmap-card">
      <div className="roadmap-card-heading">
        <CardTitle>{t.rules}</CardTitle>
        <Dialog
          open={open}
          onOpenChange={(v) => {
            if (busy) return;
            if (v) {
              setDraft(initial());
              id.current = crypto.randomUUID();
              setError('');
            }
            setOpen(v);
          }}
          trigger={<Button disabled={busy}>{t.addRule}</Button>}
          title={t.addRule}
          description={t.ruleHint}
        >
          <form
            onSubmit={async (e) => {
              e.preventDefault();
              if (await act({ type: 'rule', id: id.current, input: draft })) setOpen(false);
              else setError(t.error);
            }}
          >
            <fieldset disabled={busy} className="roadmap-fieldset">
              <FormGrid>
                {(['school', 'college', 'major'] as const).map((key, n) => (
                  <Field
                    key={key}
                    label={[t.school, t.college, t.major][n]}
                    value={draft[key]}
                    required={n === 0}
                    onChange={(v) => setDraft({ ...draft, [key]: v })}
                  />
                ))}
              </FormGrid>
              <Field
                label={t.year}
                type="number"
                min={2000}
                max={2100}
                value={draft.year}
                onChange={(v) => setDraft({ ...draft, year: Number(v) })}
              />
              <Field
                label={t.journey}
                value={draft.journey_id ?? ''}
                options={[
                  { value: '', label: t.none },
                  ...data.entities
                    .filter((e) => e.kind === 'journey' && !e.deleted_at && !e.archived_at)
                    .map((e) => ({ value: e.id, label: e.title })),
                ]}
                onChange={(v) => setDraft({ ...draft, journey_id: v || null })}
              />
              <Field
                label={t.source}
                type="url"
                value={draft.source_url}
                required
                onChange={(v) => setDraft({ ...draft, source_url: v })}
              />
              {(['written_exam', 'interview', 'body', 'notes'] as const).map((key, n) => (
                <Field
                  key={key}
                  label={[t.written, t.interview, t.body, t.notes][n]}
                  multiline
                  required={n === 2}
                  value={draft[key]}
                  onChange={(v) => setDraft({ ...draft, [key]: v })}
                />
              ))}
              <Button type="submit" disabled={busy}>
                {t.save}
              </Button>
            </fieldset>
            {error && <p role="alert">{error}</p>}
          </form>
        </Dialog>
      </div>
      <p className="muted">{t.ruleHint}</p>
      <p>
        {t.futureRule}{' '}
        <a href="https://gsao.fudan.edu.cn/15015/list.htm" target="_blank" rel="noreferrer">
          {t.official}
        </a>
      </p>
      {data.rules.length === 0 && <p className="muted">{t.ruleEmpty}</p>}
      {data.rules.map((rule) => (
        <details key={rule.id} className="roadmap-rule">
          <summary>
            {rule.input.school} · {rule.input.year} · {rule.input.college} {rule.input.major}
            {rule.deleted_at ? ' · ' + t.recycle : ''}
          </summary>
          <p>
            {t.captured}：{new Date(rule.captured_at).toLocaleString('zh-CN')}
          </p>
          <p className="roadmap-hash">
            {t.hash}：{rule.content_hash}
          </p>
          <a href={rule.input.source_url} target="_blank" rel="noreferrer">
            {rule.input.source_url}
          </a>
          <p className="preserve-lines">{rule.input.body}</p>
          <p className="preserve-lines">{rule.input.written_exam}</p>
          <p className="preserve-lines">{rule.input.interview}</p>
          <p className="preserve-lines">{rule.input.notes}</p>
          <Button
            variant="secondary"
            disabled={busy}
            onClick={() => void act({ type: 'rule_trash', id: rule.id, deleted: !rule.deleted_at })}
          >
            {rule.deleted_at ? t.restore : t.trash}
          </Button>
        </details>
      ))}
    </Card>
  );
}
