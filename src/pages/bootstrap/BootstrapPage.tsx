import { useState } from 'react';
import { verifyDesktop } from '../../lib/desktop';
import { t } from '../../lib/i18n';
import { DatabaseDiagnostics } from '../../features/database/DatabaseDiagnostics';
export function BootstrapPage() {
  const [status, setStatus] = useState('');
  const [busy, setBusy] = useState(false);
  async function verify() {
    setBusy(true);
    setStatus('');
    try {
      const info = await verifyDesktop();
      setStatus(info ? t.connected + ' · ' + info.version : t.browser);
    } catch {
      setStatus(t.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <section>
      <p className="eyebrow">LUMEN / PERSONAL GROWTH OS</p>
      <h1>{t.tagline}</h1>
      <p className="intro">{t.description}</p>
      <div className="connection">
        <span className="dot" />
        <h2>{t.bootstrap}</h2>
        <p>React · TypeScript · Tauri 2</p>
        <button onClick={verify} disabled={busy}>
          {busy ? t.checking : t.verify}
        </button>
        <p role="status" aria-label={t.connectionStatus} className="status">
          {status}
        </p>
      </div>
      <DatabaseDiagnostics />
    </section>
  );
}
