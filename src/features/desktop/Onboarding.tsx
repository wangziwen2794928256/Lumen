import { useEffect, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { Dialog } from '../../components/ui/dialog';
import { Button } from '../../components/ui/button';
import { desktopText as t } from './text';
import type { DesktopStatus } from './model';
export function Onboarding() {
  const [open, setOpen] = useState(false),
    [step, setStep] = useState(0),
    [journey, setJourney] = useState(''),
    [subjects, setSubjects] = useState(''),
    [weekday, setWeekday] = useState(210),
    [weekend, setWeekend] = useState(210),
    [shortcut, setShortcut] = useState(''),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(''),
    [saved, setSaved] = useState(false);
  useEffect(() => {
    if (!isTauri()) return;
    let alive = true;
    void invoke<DesktopStatus>('desktop_status')
      .then((s) => {
        if (alive) setOpen(s.onboarding);
      })
      .catch(() => {});
    return () => {
      alive = false;
    };
  }, []);
  const names = subjects
    .split('\n')
    .map((s) => s.trim())
    .filter(Boolean);
  const valid =
    journey.length <= 250 &&
    Number.isInteger(weekday) &&
    weekday >= 0 &&
    weekday <= 1440 &&
    Number.isInteger(weekend) &&
    weekend >= 0 &&
    weekend <= 1440 &&
    names.length <= 12 &&
    names.every((s) => s.length <= 250) &&
    shortcut.length <= 80;
  async function skip() {
    if (busy) return;
    setBusy(true);
    setError('');
    try {
      await invoke('onboarding_skip');
      setOpen(false);
    } catch {
      setError(t.onboardingError);
    } finally {
      setBusy(false);
    }
  }
  async function complete() {
    if (saved) {
      setOpen(false);
      return;
    }
    setBusy(true);
    setError('');
    try {
      await invoke('onboarding_complete', {
        input: { journey, subjects: names, weekday_minutes: weekday, weekend_minutes: weekend },
      });
      setSaved(true);
      if (shortcut.trim()) {
        try {
          await invoke('capture_shortcut', { shortcut: shortcut.trim() });
        } catch {
          setError(t.shortcutError);
          return;
        }
      }
      setOpen(false);
    } catch {
      setError(t.onboardingError);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Dialog
      trigger={<button hidden aria-hidden="true" tabIndex={-1} />}
      title={t.welcome}
      description={t.onboardingHint}
      open={open}
      onOpenChange={(v) => {
        if (!v) void skip();
      }}
    >
      <div className="onboarding-stack">
        <p>{t.step(step + 1)}</p>
        {step === 0 && (
          <label>
            {t.journey}
            <input value={journey} maxLength={250} onChange={(e) => setJourney(e.target.value)} />
          </label>
        )}
        {step === 1 && (
          <>
            <label>
              {t.weekday}
              <input
                type="number"
                min={0}
                max={1440}
                value={weekday}
                onChange={(e) => setWeekday(e.target.valueAsNumber)}
              />
            </label>
            <label>
              {t.weekend}
              <input
                type="number"
                min={0}
                max={1440}
                value={weekend}
                onChange={(e) => setWeekend(e.target.valueAsNumber)}
              />
            </label>
            <p>{t.capacity(weekday * 5 + weekend * 2)}</p>
          </>
        )}
        {step === 2 && (
          <label>
            {t.subjects}
            <textarea
              rows={5}
              value={subjects}
              onChange={(e) => setSubjects(e.target.value)}
              maxLength={3012}
            />
          </label>
        )}
        {step === 3 && (
          <>
            <label>
              {t.shortcut}
              <input
                value={shortcut}
                onChange={(e) => setShortcut(e.target.value)}
                maxLength={80}
              />
            </label>
            <p>{t.shortcutHint}</p>
          </>
        )}
        {step === 4 && (
          <>
            <p>{t.summary(journey, new Set(names).size, weekday * 5 + weekend * 2)}</p>
            <p>{t.integrationsHint}</p>
          </>
        )}
        {error && <p role="alert">{error}</p>}
        <div className="desktop-actions">
          <Button variant="secondary" disabled={busy} onClick={() => void skip()}>
            {t.skip}
          </Button>
          {step > 0 && (
            <Button
              variant="secondary"
              disabled={busy || saved}
              onClick={() => setStep((s) => s - 1)}
            >
              {t.back}
            </Button>
          )}
          {step < 4 ? (
            <Button disabled={busy || !valid} onClick={() => setStep((s) => s + 1)}>
              {t.next}
            </Button>
          ) : (
            <Button disabled={busy || !valid} onClick={() => void complete()}>
              {t.enter}
            </Button>
          )}
        </div>
      </div>
    </Dialog>
  );
}
