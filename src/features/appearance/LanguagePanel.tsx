import { useState } from 'react';
import { locale, persistLocale, type Locale } from '../../lib/locale';
import { Button } from '../../components/ui/button';
export function LanguagePanel() {
  const [selection, setSelection] = useState<Locale>(locale);
  const [failed, setFailed] = useState(false);
  return (
    <div className="language-panel">
      <label htmlFor="interface-language">
        {locale === 'en' ? 'Interface language' : '界面语言'}
      </label>
      <select
        id="interface-language"
        value={selection}
        onChange={(e) => setSelection(e.target.value as Locale)}
      >
        <option value="zh-CN">简体中文</option>
        <option value="en">English</option>
      </select>
      <p>
        {locale === 'en'
          ? 'Save your work before applying. Lumen reloads the interface. Your own content stays in its original language.'
          : '应用前请保存当前编辑，界面将重新载入。你写入的内容保持原文。'}
      </p>
      <Button
        disabled={selection === locale}
        onClick={() => {
          if (persistLocale(selection)) window.location.reload();
          else setFailed(true);
        }}
      >
        {locale === 'en' ? 'Apply language and reload' : '应用语言并重新载入'}
      </Button>
      {failed && (
        <p role="alert">
          {locale === 'en'
            ? 'Language preference could not be saved. Check local storage access.'
            : '语言设置无法保存，请检查本地存储权限。'}
        </p>
      )}
    </div>
  );
}
