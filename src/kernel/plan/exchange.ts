import { z } from 'zod';
export const planExchangeSchema = z
  .object({
    schema_version: z.literal('1.0'),
    request_id: z.uuid(),
    basis_hash: z.string().regex(/^[0-9a-fA-F]{64}$/),
    source: z.enum(['manual', 'external-proposal', 'local-proposal']),
    capacity_minutes: z.number().int().min(0).max(10080),
    items: z
      .array(
        z
          .object({
            task_id: z.uuid(),
            date: z.iso.date(),
            minutes: z.number().int().min(1).max(1440),
          })
          .strict(),
      )
      .max(500),
  })
  .strict()
  .refine((p) => p.items.reduce((n, i) => n + i.minutes, 0) <= p.capacity_minutes, {
    message: 'CAPACITY_EXCEEDED',
  })
  .refine((p) => new Set(p.items.map((i) => i.task_id + '|' + i.date)).size === p.items.length, {
    message: 'DUPLICATE_BLOCK',
  });
export type PlanExchange = z.infer<typeof planExchangeSchema>;
export const planContextSchema = z
  .object({
    schema_version: z.literal('1.0'),
    request_id: z.uuid(),
    basis_hash: z.string().regex(/^[0-9a-fA-F]{64}$/),
    tasks: z
      .array(
        z
          .object({
            task_id: z.uuid(),
            revision: z.number().int().nonnegative(),
            estimate_minutes: z.number().int().min(1).max(1440),
            actual_ms: z.number().int().nonnegative(),
            due_date: z.iso.date(),
            label: z.string().max(1000).optional(),
          })
          .strict(),
      )
      .max(500),
    capacities: z
      .array(z.object({ date: z.iso.date(), minutes: z.number().int().min(0).max(1440) }).strict())
      .max(31),
  })
  .strict();
export type PlanContext = z.infer<typeof planContextSchema>;
// Data-only contract. No AI invocation or automatic acceptance exists.
