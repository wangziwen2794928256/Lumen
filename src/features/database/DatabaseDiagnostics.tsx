import { useState } from 'react';
import { verifyDatabase, type DatabaseStatus } from '../../lib/desktop';
import { dataText as d } from '../../lib/data-i18n';
import { Card, CardHeader, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
export function DatabaseDiagnostics() {
  const [status, setStatus] = useState('');
  const [data, setData] = useState<DatabaseStatus | null>(null);
  const [busy, setBusy] = useState(false);
  async function check() {
    setBusy(true);
    setData(null);
    setStatus('');
    try {
      const result = await verifyDatabase();
      setData(result);
      setStatus(result?.integrity === 'ok' ? d.ready : result ? d.error : d.browser);
    } catch {
      setStatus(d.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card>
      <CardHeader>
        <CardTitle>{d.heading}</CardTitle>
      </CardHeader>
      <p className="secondary-text">{d.hint}</p>
      <Button onClick={check} disabled={busy}>
        {busy ? d.busy : d.check}
      </Button>
      <p role="status" aria-label={d.status}>
        {status}
      </p>
      {data && (
        <p>
          {d.schema} {data.schema_version} · {d.records}：{d.task} {data.task_count} / {d.journal}{' '}
          {data.journal_count}
        </p>
      )}
    </Card>
  );
}
