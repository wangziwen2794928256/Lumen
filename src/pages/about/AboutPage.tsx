import { t } from '../../lib/i18n';
export function AboutPage() {
  return (
    <section>
      <p className="eyebrow">LUMEN / ENGINEERING</p>
      <h1>{t.aboutTitle}</h1>
      <p className="intro">{t.aboutDescription}</p>
      <p className="note">DATA SAFETY → RELIABILITY → USABILITY</p>
    </section>
  );
}
