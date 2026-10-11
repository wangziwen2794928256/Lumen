import { z } from 'zod';
export const manifestSchema = z
  .object({
    id: z
      .string()
      .regex(/^[a-z][a-z0-9]*(\.[a-z][a-z0-9-]*)+$/)
      .max(80),
    name: z.object({ 'zh-CN': z.string().min(1).max(80), en: z.string().min(1).max(80) }).strict(),
    version: z.string().regex(/^\d+\.\d+\.\d+$/),
    api_version: z.literal('1.0'),
    layer: z.enum(['kernel', 'official', 'community', 'theme']),
    route: z.string().regex(/^\/[a-z0-9/-]*$/),
    icon: z.string().max(40),
    required: z.boolean(),
    status: z.enum(['stable', 'module', 'legacy-bridge', 'planned']),
    permissions: z.array(z.string()).max(16),
  })
  .strict();
export type Manifest = z.infer<typeof manifestSchema>;
export interface ModulePreference {
  id: string;
  enabled: boolean;
  hidden: boolean;
  pinned: boolean;
  order: number;
}
export interface RegistryState {
  revision: number;
  items: ModulePreference[];
}
export const registryStateSchema = z
  .object({
    revision: z.number().int().nonnegative(),
    items: z
      .array(
        z
          .object({
            id: z.string(),
            enabled: z.boolean(),
            hidden: z.boolean(),
            pinned: z.boolean(),
            order: z.number().int().min(0).max(1000),
          })
          .strict(),
      )
      .max(32),
  })
  .strict();
