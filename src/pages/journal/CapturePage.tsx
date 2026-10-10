import { useState } from 'react';
import { isTauri } from '@tauri-apps/api/core';
import { Editor } from '../../features/journal/Editor';
import { asCopy, fresh, readPending } from '../../features/journal/model';
import { journalInvoke } from '../../features/journal/api';
import { journalText as t } from '../../features/journal/text';
import { Button } from '../../components/ui/button';
export function CapturePage() {
  const [value, setValue] = useState(() => fresh(true)),
    [pending, setPending] = useState(() => readPending(true)),
    [message, setMessage] = useState('');
  return (
    <main className="capture-page">
      <h1>{t.capture}</h1>
      {!isTauri() ? (
        <p>{t.desktop}</p>
      ) : (
        <>
          {pending && (
            <div role="status">
              <p>{t.recovery}</p>
              <Button
                onClick={() => {
                  setValue(asCopy(pending));
                  setPending(null);
                }}
              >
                {t.recover}
              </Button>
            </div>
          )}
          {message && <p role="status">{message}</p>}
          <Editor
            key={value.client_id + '-' + value.id}
            initial={value}
            quick
            onBack={() => void journalInvoke('capture_close')}
            onSaved={() => {
              setMessage(t.savedQuick);
              setValue(fresh(true));
            }}
            onRecover={setValue}
            onReload={() => setValue(fresh(true))}
          />
        </>
      )}
    </main>
  );
}
