import { useId, type ReactNode } from 'react';
import { Input, Select, Textarea } from '../../components/ui/input';
export function Field({
  label,
  value,
  onChange,
  type = 'text',
  min,
  max,
  options,
  multiline = false,
  required = false,
}: {
  label: string;
  value: string | number;
  onChange: (value: string) => void;
  type?: string;
  min?: number;
  max?: number;
  options?: { value: string; label: string }[];
  multiline?: boolean;
  required?: boolean;
}) {
  const id = useId();
  const props = {
    id,
    value,
    onChange: (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) =>
      onChange(e.target.value),
    required,
  };
  return (
    <div className="roadmap-field">
      <label htmlFor={id} className="field-label">
        {label}
      </label>
      {options ? (
        <Select {...props}>
          {options.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </Select>
      ) : multiline ? (
        <Textarea {...props} rows={3} maxLength={100000} />
      ) : (
        <Input {...props} type={type} min={min} max={max} />
      )}
    </div>
  );
}
export function FormGrid({ children }: { children: ReactNode }) {
  return <div className="roadmap-form-grid">{children}</div>;
}
