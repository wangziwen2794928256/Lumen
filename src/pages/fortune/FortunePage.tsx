import { FortuneCard } from '../../features/fortune/FortuneCard';
import { fortuneText as t } from '../../features/fortune/text';
export function FortunePage() {
  return (
    <div className="fortune-page">
      <header className="page-heading">
        <div>
          <p className="eyebrow">{t.eyebrow}</p>
          <h1>{t.heading}</h1>
          <p className="secondary-text">{t.intro}</p>
        </div>
      </header>
      <FortuneCard expanded />
    </div>
  );
}
