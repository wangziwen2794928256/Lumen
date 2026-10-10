import { Card, CardHeader, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Input, Textarea, Select } from '../../components/ui/input';
import { Dialog, DialogClose } from '../../components/ui/dialog';
import { Badge } from '../../components/ui/badge';
import { Link } from 'react-router-dom';
import { designText as d } from '../../lib/design-i18n';
export function DesignPage() {
  return (
    <>
      <div className="page-heading">
        <div>
          <p className="eyebrow">FOUNDATIONS / {d.library}</p>
          <h1>{d.libraryTitle}</h1>
          <p className="intro">{d.libraryHint}</p>
        </div>
        <Badge>{d.preview}</Badge>
      </div>
      <div className="design-grid">
        <Card className="span-full">
          <CardHeader>
            <CardTitle>{d.appearance}</CardTitle>
          </CardHeader>
          <p className="secondary-text">{d.libraryAppearanceHint}</p>
          <Button asChild variant="secondary">
            <Link to="/settings">{d.openAppearanceSettings}</Link>
          </Button>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>{d.buttons}</CardTitle>
          </CardHeader>
          <div className="button-samples">
            <Button>{d.primary}</Button>
            <Button variant="secondary">{d.secondary}</Button>
            <Button variant="ghost">{d.ghost}</Button>
            <Button variant="destructive">{d.danger}</Button>
            <Button disabled>{d.disabled}</Button>
          </div>
          <Dialog
            trigger={<Button variant="secondary">{d.dialog}</Button>}
            title={d.dialogTitle}
            description={d.dialogDescription}
          >
            <label htmlFor="dialog-title" className="field-label">
              {d.field}
            </label>
            <Input id="dialog-title" placeholder={d.placeholder} />
            <label htmlFor="dialog-notes" className="field-label">
              {d.details}
            </label>
            <Textarea id="dialog-notes" rows={3} />
            <div className="dialog-actions">
              <DialogClose asChild>
                <Button variant="secondary">{d.cancel}</Button>
              </DialogClose>
            </div>
          </Dialog>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>{d.form}</CardTitle>
          </CardHeader>
          <label htmlFor="sample-title" className="field-label">
            {d.field}
          </label>
          <Input id="sample-title" placeholder={d.placeholder} />
          <label htmlFor="sample-type" className="field-label">
            {d.select}
          </label>
          <Select id="sample-type">
            <option>{d.reflection}</option>
            <option>{d.note}</option>
          </Select>
          <label htmlFor="sample-error" className="field-label">
            {d.error}
          </label>
          <Input id="sample-error" aria-invalid="true" aria-describedby="sample-error-message" />
          <p id="sample-error-message" className="field-error">
            {d.errorText}
          </p>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>{d.tokens}</CardTitle>
          </CardHeader>
          <div className="token-grid">
            {[
              'background',
              'surface',
              'surface-raised',
              'accent-primary',
              'success',
              'warning',
              'danger',
              'border',
            ].map((token) => (
              <div key={token}>
                <span style={{ background: 'var(--' + token + ')' }} />
                <code>{token}</code>
              </div>
            ))}
          </div>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>{d.typography}</CardTitle>
          </CardHeader>
          <h3 className="type-sample">{d.typeTitle}</h3>
          <p>{d.bodyText}</p>
          <p className="secondary-text">{d.mutedText}</p>
          <div className="button-samples">
            <Badge>{d.planned}</Badge>
            <Badge>{d.actual}</Badge>
            <Badge>{d.empty}</Badge>
          </div>
        </Card>
      </div>
    </>
  );
}
