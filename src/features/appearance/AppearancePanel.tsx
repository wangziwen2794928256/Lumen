import { Sun, Moon, Monitor } from 'lucide-react';
import { useAppearance } from '../../app/providers/appearance-context';
import { designText as d } from '../../lib/design-i18n';
import type { Accent, ThemeMode } from '../../lib/appearance';
import { Button } from '../../components/ui/button';
const modes = [
  ['light', d.light, Sun],
  ['dark', d.dark, Moon],
  ['system', d.system, Monitor],
] as const;
const accents = [
  ['sage', d.sage],
  ['blue', d.blue],
  ['lavender', d.lavender],
  ['gray', d.gray],
] as const;
export function AppearancePanel() {
  const { value, update } = useAppearance();
  return (
    <div className="appearance-panel">
      <fieldset>
        <legend>{d.mode}</legend>
        <div className="choice-row">
          {modes.map(([mode, label, Icon]) => (
            <Button
              key={mode}
              variant={value.mode === mode ? 'default' : 'secondary'}
              aria-pressed={value.mode === mode}
              onClick={() => update({ mode: mode as ThemeMode })}
            >
              <Icon size={16} aria-hidden="true" />
              {label}
            </Button>
          ))}
        </div>
      </fieldset>
      <fieldset>
        <legend>{d.accent}</legend>
        <div className="choice-row">
          {accents.map(([accent, label]) => (
            <Button
              key={accent}
              variant="secondary"
              aria-pressed={value.accent === accent}
              onClick={() => update({ accent: accent as Accent })}
            >
              <span className={'accent-dot ' + accent} />
              {label}
            </Button>
          ))}
        </div>
      </fieldset>
    </div>
  );
}
