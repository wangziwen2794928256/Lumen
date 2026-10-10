import { useState } from 'react';
import Markdown from 'react-markdown';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from '../roadmap/Field';
import { useDraft } from './useDraft';
import { Attachments } from './Attachments';
import { entryTypes, journalText as t } from './text';
import { asCopy, type Choice, type Save } from './model';
export function Editor({
  initial,
  quick = false,
  choices,
  onBack,
  onSaved,
  onRecover,
  onReload,
}: {
  initial: Save;
  quick?: boolean;
  choices?: { journeys: Choice[]; tasks: Choice[]; music: Choice[] };
  onBack: () => void;
  onSaved: () => void;
  onRecover: (s: Save) => void;
  onReload: () => void;
}) {
  const d = useDraft(initial, quick),
    [preview, setPreview] = useState(false),
    [published, setPublished] = useState(initial.revision > 0);
  const v = d.value;
  const [tagsText, setTagsText] = useState(initial.tags.join(', '));
  async function save() {
    if (await d.publish()) {
      setPublished(true);
      onSaved();
    }
  }
  async function back() {
    if (await d.flush()) onBack();
  }
  function choice(label: string, field: 'journey_id' | 'task_id' | 'music_id', items: Choice[]) {
    return (
      <Field
        label={label}
        value={v[field] ?? ''}
        onChange={(x) => d.change({ [field]: x || null })}
        options={[
          { value: '', label: t.none },
          ...items.map((x) => ({ value: x.id, label: x.title })),
        ]}
      />
    );
  }
  return (
    <div className="journal-editor">
      <div className="journal-toolbar">
        <span>{t.private}</span>
        <Button variant="ghost" onClick={() => void back()}>
          {quick ? t.close : t.back}
        </Button>
      </div>
      <FormGrid>
        {!quick && (
          <Field label={t.titleField} value={v.title} onChange={(title) => d.change({ title })} />
        )}
        <Field
          label={t.type}
          value={v.entry_type}
          onChange={(entry_type) => d.change({ entry_type: entry_type as Save['entry_type'] })}
          options={entryTypes}
        />
        {!quick && (
          <Field
            label={t.date}
            value={v.date}
            type="date"
            onChange={(date) => d.change({ date })}
          />
        )}
        <Field
          label={t.tags}
          value={tagsText}
          onChange={(tags) => {
            setTagsText(tags);
            d.change({
              tags: tags
                .split(/[,，]/)
                .map((x) => x.trim())
                .filter(Boolean),
            });
          }}
        />
      </FormGrid>
      <div className="journal-toolbar">
        <Button variant="secondary" aria-pressed={preview} onClick={() => setPreview(!preview)}>
          {preview ? t.edit : t.preview}
        </Button>
        <span role="status">{d.status}</span>
      </div>
      {preview ? (
        <div className="journal-markdown">
          <Markdown
            skipHtml
            components={{ img: ({ alt }) => <span>{alt ?? ''}</span> }}
            urlTransform={(url) => (/^(https?:\/\/|#)/i.test(url) ? url : '')}
          >
            {v.body}
          </Markdown>
        </div>
      ) : (
        <div className="journal-body">
          <label htmlFor={'journal-body-' + v.id}>{t.body}</label>
          <textarea
            id={'journal-body-' + v.id}
            autoFocus={quick}
            value={v.body}
            maxLength={1000000}
            onChange={(e) => d.change({ body: e.target.value })}
          />
        </div>
      )}
      {!quick && choices && (
        <FormGrid>
          {choice(t.journey, 'journey_id', choices.journeys)}
          {choice(t.task, 'task_id', choices.tasks)}
          {choice(t.music, 'music_id', choices.music)}
        </FormGrid>
      )}
      {d.error && (
        <div role="alert">
          <p>{d.error}</p>
          <Button variant="secondary" onClick={() => onRecover(asCopy(v))}>
            {t.recover}
          </Button>
          {initial.revision > 0 && (
            <Button variant="ghost" onClick={onReload}>
              {t.reload}
            </Button>
          )}
        </div>
      )}
      <Button onClick={() => void save()} disabled={!v.title.trim() && !v.body.trim()}>
        {t.save}
      </Button>
      {published && !quick && <Attachments id={v.id} />}
    </div>
  );
}
