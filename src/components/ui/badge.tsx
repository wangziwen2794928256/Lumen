import type { ComponentProps } from 'react';
import { cn } from '../../lib/utils';
export function Badge({ className, ...props }: ComponentProps<'span'>) {
  return <span className={cn('ui-badge', className)} {...props} />;
}
