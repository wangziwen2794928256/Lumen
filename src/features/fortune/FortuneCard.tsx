import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Sparkles } from 'lucide-react';
import { Card, CardTitle } from '../../components/ui/card';
import { dailyIndex, localDay } from './model';
import { fortuneText as t } from './text';
export function FortuneCard({ expanded = false }: { expanded?: boolean }) {
  const [day, setDay] = useState(localDay);
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout>;
    const refresh = () => {
      clearTimeout(timer);
      const now = new Date();
      setDay(localDay(now));
      const midnight = new Date(now);
      midnight.setHours(24, 0, 0, 100);
      // Recheck periodically to notice timezone/clock changes after resume.
      timer = setTimeout(refresh, Math.min(midnight.getTime() - now.getTime(), 60000));
    };
    refresh();
    window.addEventListener('focus', refresh);
    document.addEventListener('visibilitychange', refresh);
    return () => {
      clearTimeout(timer);
      window.removeEventListener('focus', refresh);
      document.removeEventListener('visibilitychange', refresh);
    };
  }, []);
  const [title, description, suggestion] = t.signs[dailyIndex(day, t.signs.length)];
  return (
    <Card className={'fortune-card' + (expanded ? ' fortune-expanded' : '')}>
      <div className="fortune-heading">
        <CardTitle>
          <Sparkles size={18} aria-hidden="true" /> {t.heading}
        </CardTitle>
        <time dateTime={day}>{day}</time>
      </div>
      <p className="fortune-sign">{title}</p>
      <p>{description}</p>
      {expanded && (
        <div className="fortune-suggestion">
          <h3>{t.suggestion}</h3>
          <p>{suggestion}</p>
        </div>
      )}
      <p className="secondary-text fortune-note">{t.note}</p>
      {expanded ? (
        <p className="secondary-text">{t.privacy}</p>
      ) : (
        <Link to="/fortune">{t.details} →</Link>
      )}
      {expanded && <Link to="/settings">{t.settings} →</Link>}
    </Card>
  );
}
