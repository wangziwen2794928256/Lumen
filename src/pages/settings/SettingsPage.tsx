import { LanguagePanel } from '../../features/appearance/LanguagePanel';
import { translate } from '../../lib/locale';
import { type ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { ModuleSettings } from '../../kernel/modules/ModuleSettings';
import { BackupSettings } from '../../features/privacy/BackupSettings';
import { PrivacyCenter } from '../../features/privacy/PrivacyCenter';
import { DesktopSettings } from '../../features/desktop/DesktopSettings';
import { musicText } from '../../features/music/text';
import { AppearancePanel } from '../../features/appearance/AppearancePanel';
import { CaptureSettings } from '../../features/journal/CaptureSettings';
import { PlanningSettings } from '../../features/roadmap/PlanningSettings';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { designText as d } from '../../lib/design-i18n';
import { groups, settingsText as t } from './text';
function Group({ id, title, children }: { id: string; title: string; children: ReactNode }) {
  return (
    <section className="settings-group" aria-labelledby={`settings-${id}`}>
      <h2 id={`settings-${id}`} tabIndex={-1}>
        {title}
      </h2>
      {children}
    </section>
  );
}
export function SettingsPage() {
  return (
    <div className="settings-stack">
      <div className="page-heading">
        <div>
          <p className="eyebrow">SETTINGS / {d.settings}</p>
          <h1>{d.settings}</h1>
          <p className="intro">{t.copy0}</p>
        </div>
      </div>
      <nav className="settings-index" aria-label={t.copy9}>
        {groups.map(([id, name]) => (
          <Button
            key={id}
            variant="secondary"
            onClick={() => {
              const heading = document.getElementById(`settings-${id}`);
              heading?.focus({ preventScroll: true });
              heading?.scrollIntoView({ block: 'start' });
            }}
          >
            {name}
          </Button>
        ))}
      </nav>
      <Group id="general" title={t.copy10}>
        <DesktopSettings />
      </Group>
      <Group id="appearance" title={t.copy11}>
        <Card>
          <LanguagePanel />
          <AppearancePanel />
        </Card>
      </Group>
      <Group id="study" title={t.copy12}>
        <PlanningSettings />
      </Group>
      <Group id="timer" title={t.copy13}>
        <Card>
          <p>{t.copy1}</p>
          <Link to="/focus">{t.copy2}</Link>
        </Card>
      </Group>
      <Group id="calendar" title={t.copy14}>
        <Card>
          <p>{t.copy3}</p>
          <Link to="/calendar">{t.copy4}</Link>
        </Card>
      </Group>
      <Group id="music" title={t.copy15}>
        <Card>
          <p>{t.copy5}</p>
          <Link to="/music">{t.copy6}</Link>
        </Card>
      </Group>
      <Group id="integrations" title={t.copy16}>
        <Card>
          <p>{musicText.spotify}</p>
          <p>{t.copy7}</p>
        </Card>
      </Group>
      <Group id="privacy" title={t.copy17}>
        <PrivacyCenter />
      </Group>
      <Group id="backup" title={t.copy18}>
        <BackupSettings />
      </Group>
      <Group id="data" title={t.copy19}>
        <Card>
          <p>{t.dataHint}</p>
          <Button
            variant="secondary"
            onClick={() => {
              document.getElementById('settings-privacy')?.focus();
              document.getElementById('settings-privacy')?.scrollIntoView({ block: 'start' });
            }}
          >
            {translate('前往数据操作')}
          </Button>
        </Card>
      </Group>
      <Group id="shortcuts" title={t.copy20}>
        <CaptureSettings />
      </Group>
      <Group id="advanced" title={t.copy21}>
        <ModuleSettings />
      </Group>
      <Group id="about" title={t.copy22}>
        <Card>
          <p>Lumen · Local-first Personal Growth Operating System · v0.1.0</p>
          <Link to="/about">{t.copy8}</Link>
        </Card>
      </Group>
    </div>
  );
}
