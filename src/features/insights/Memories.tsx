import { translate } from '../../lib/locale';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { insightsApi } from './api';
import { today, recordHref, type RecordLink } from './model';
export function Memories() {
  const [items, setItems] = useState<RecordLink[]>([]);
  useEffect(() => {
    let live = true;
    insightsApi
      .memories(today())
      .then((r) => {
        if (live) setItems(r);
      })
      .catch(() => {});
    return () => {
      live = false;
    };
  }, []);
  if (items.length === 0) return null;
  return (
    <section className="ui-card insight-memory">
      <h2>{translate('往年今日')}</h2>
      <ul>
        {items.map((r) => (
          <li key={`${r.kind}-${r.id}-${r.date}`}>
            <Link to={recordHref(r)}>
              {r.date} · {r.title}
            </Link>
          </li>
        ))}
      </ul>
      <p className="secondary-text">{translate('可在洞察的活动配置中关闭。')}</p>
    </section>
  );
}
