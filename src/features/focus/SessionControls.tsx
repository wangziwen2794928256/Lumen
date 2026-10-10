import { Button } from '../../components/ui/button';
import { Badge } from '../../components/ui/badge';
import { duration, type FocusSession, type TodayAction } from '../today/model';
import { todayText as t } from '../today/text';
import { focusText as f } from './text';
export function SessionControls({
  session,
  busy,
  blocked,
  details = false,
  act,
}: {
  session: FocusSession;
  busy: boolean;
  blocked: boolean;
  details?: boolean;
  act: (action: TodayAction) => Promise<boolean>;
}) {
  const send = (operation: 'pause' | 'resume' | 'finish' | 'cancel' | 'interrupt') =>
    void act({ type: 'session', id: session.id, operation });
  return (
    <>
      <Badge>{t[session.state]}</Badge>
      <p className="focus-value" aria-label={t.elapsed}>
        {duration(session.elapsed_ms / 1000)}
      </p>
      {details && (
        <dl className="focus-statistics">
          {[
            [f.planned, session.planned_seconds],
            [f.paused, session.paused_ms / 1000],
            [f.interrupted, session.interrupted_ms / 1000],
          ].map(([label, value]) => (
            <div key={label}>
              <dt>{label}</dt>
              <dd>{duration(Number(value))}</dd>
            </div>
          ))}
        </dl>
      )}
      {session.state === 'interrupted' && (
        <p className="preview-notice">
          {session.interruption_reason && f.reasons[session.interruption_reason]} {t.recovery}
        </p>
      )}
      {blocked && (
        <p role="status" className="preview-notice">
          {f.blocked}
        </p>
      )}
      <div className="task-actions">
        {session.state === 'running' ? (
          <Button disabled={busy} onClick={() => send('pause')}>
            {t.pause}
          </Button>
        ) : (
          <Button disabled={busy || blocked} onClick={() => send('resume')}>
            {t.resume}
          </Button>
        )}
        <Button variant="secondary" disabled={busy} onClick={() => send('finish')}>
          {session.state === 'interrupted' ? t.finishLast : t.finish}
        </Button>
        <Button
          variant="ghost"
          disabled={busy || session.state === 'interrupted'}
          onClick={() => send('interrupt')}
        >
          {t.interrupt}
        </Button>
        <Button variant="ghost" disabled={busy} onClick={() => send('cancel')}>
          {details ? f.discard : t.cancel}
        </Button>
      </div>
      {details && <p className="secondary-text">{f.discardHint}</p>}
    </>
  );
}
