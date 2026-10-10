import { Card } from '../../components/ui/card';
import { Badge } from '../../components/ui/badge';
import { designText as d } from '../../lib/design-i18n';
export function ModulePreview({ title }: { title: string }) {
  return (
    <>
      <div className="page-heading">
        <h1>{title}</h1>
        <Badge>{d.preview}</Badge>
      </div>
      <Card>
        <p className="intro">{d.pending}</p>
      </Card>
    </>
  );
}
