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
export const moduleText = {
  heading: '模块管理',
  hint: '启用控制页面和模块入口；隐藏只改变侧栏。置顶模块优先显示。拖动排列，或用上下按钮。关闭模块不会删除记录。',
  enabled: '启用',
  hidden: '隐藏',
  pinned: '置顶',
  up: '上移',
  down: '下移',
  save: '保存模块配置',
  error: '保存失败或版本已变化；当前选择已保留，请重新载入后重试。',
  reload: '重新载入模块配置',
  disabled: '此模块已停用，可在设置中重新启用。',
  saved: '模块配置已保存',
  required: '核心模块不能停用；安全系统始终属于 Kernel。',
};
