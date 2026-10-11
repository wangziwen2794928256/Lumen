# 实作：编写第一个官方模块

本教程把 `lumen.study-lab` 作为可信源码模块接入应用，运行权限与现有官方模块相同。它不允许执行不可信社区代码。请在 fork 和 feature 分支中操作；开发启动必须使用隔离 OS 用户或 QA identifier，不能拿生产 AppData 试验。

## 1. 准备环境

先完成 [环境配置](ENVIRONMENT_AND_CONFIGURATION.md)，执行 `pnpm install --frozen-lockfile`，阅读 [模块清单](MODULE_REFERENCE.md) 和 [API 参考](API_REFERENCE.md)。第一个模块只显示页面，不写库，因此无需 migration 或新权限。

## 2. 编写页面入口

创建 `src/modules/official/study-lab/index.tsx`：

```tsx
import { Card } from '../../../components/ui/card';
import { locale } from '../../../lib/locale';
export function StudyLabRoute() {
  return <Card>
    <h1>{locale === 'en' ? 'Study Lab' : '学习实验室'}</h1>
    <p>{locale === 'en' ? 'Build a workflow that fits your goal.' : '为自己的目标编写工作流。'}</p>
  </Card>;
}
```

页面复杂后放入 `src/pages/study-lab`，在入口使用 lazy 导出，参考 `src/modules/official/profile/index.tsx`。保留既有 Service，不复制业务写入。正式文案应补入语言资源；用户自己的内容不翻译。

## 3. 声明 Manifest

向 `src/kernel/modules/catalogue.json` 追加：

```json
{
  "id": "lumen.study-lab",
  "name": { "zh-CN": "学习实验室", "en": "Study Lab" },
  "version": "0.1.0",
  "api_version": "1.0",
  "layer": "official",
  "route": "/study-lab",
  "icon": "NotebookPen",
  "required": false,
  "status": "module",
  "permissions": []
}
```

`validateCatalogue` 只接受捆绑的 `lumen.*` kernel/official 条目。ID 与 route 必须唯一；API 固定 `1.0`；名称必须有中英两部分；route 只用允许的小写路径字符。图标从 AppShell 映射取值，未知图标安全回落。strict schema 拒绝额外字段。Manifest 声明导航配置，不负责动态加载代码。

## 4. 显式接入可信路由

在 `src/app/router.tsx` 导入：

```tsx
import { StudyLabRoute } from '../modules/official/study-lab';
```

在既有 `<Routes>` 内添加：

```tsx
<Route path="/study-lab" element={
  <ModuleGate id="lumen.study-lab"><StudyLabRoute /></ModuleGate>
} />
```

此时 Sidebar 从 Registry 自动出现，执行代码仍是构建时显式 import。`normalizeState` 为新增模块补默认项，保留旧偏好。停用拦住页面，隐藏只改变侧栏；置顶组优先，拖动只能在相同 pinned 分组内重排。保留上下移动按钮与乐观版本冲突提示。若使用 lazy，可参考 `src/app/preload.ts` 增加预取；预取不可产生业务写入。

## 5. 可选：首页 Widget

查看 `src/modules/official/insights/index.tsx`，现有 `insightsWidgets` 在 `today.after-summary` 槽位贡献组件。描述器字段为 `id/moduleId/slot/order/Component`。在新模块导出同形数组，然后在 `src/kernel/modules/Extensions.tsx` 显式导入，与现有数组合并后再执行 filter/sort。当前是可信静态贡献列表，不是通用第三方加载器。

保留 `TodayWidgets` 的启用过滤及逐组件 `WidgetBoundary`。单个 widget 失败不应拖垮首页。只使用明确需要的 typed client，不向模块传任意文件路径、SQL、HTML script 或后台自动写权限。

## 6. 确需保存数据时

只读页面不改 schema。新增事实存储必须在 010 后追加下一编号 migration，在 `database/migrations.rs` 注册，说明列/索引/关联，并在 TempDir 测试升级和失败回滚。不得修改已发布 001–010。Kernel 独占核心数据库访问；先建立 typed command、Rust 校验、参数化仓储、请求幂等与 revision 策略，再开放前端 client。

OS 分支只能进入 Kernel platform adapter，缺能力返回显式错误。社区插件不继承这些官方 IPC 权限。

## 7. 验证接入

```sh
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
pnpm test:e2e
node --experimental-strip-types scripts/check-contracts.mjs
```

验收 manifest/route 唯一性、新条目默认补齐、停用路由拦截、hidden/pinned/order 持久化、键盘重排、widget 失败隔离、停用再启用后的记录保留。中英文、浅深色、三种窗口尺寸均检查。新业务或存储补 Rust 测试。浏览器 fixture 不代替原生 IPC 与 OS 集成验收。

## 常见接入问题

| 现象 | 检查 |
| --- | --- |
| Registry 初始化报错 | API、命名空间、重复 route/id、额外 manifest 字段 |
| 侧栏有入口，页面跳走 | router.tsx 没有显式 Route |
| 侧栏没有入口 | hidden=true 或 enabled=false |
| 页面显示停用 | 在设置启用并保存 |
| 保存冲突 | 重新载入 Registry，再明确重做选择 |
| 首页 widget 不显示 | 数组是否导入合并、slot 拼写、模块 enabled |
| 新模块改坏旧记录 | 停止写入，检查迁移、事务及数据所有权 |

社区插件接入见 [Plugin SDK](PLUGIN_SDK.md)。当前 enable 返回 `PLUGIN_RUNTIME_DEFERRED`；修改数据库状态也不能授予执行能力。
