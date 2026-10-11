import { translate } from '../../lib/locale';
import { Component, type ReactNode } from 'react';
import { fortuneWidgets } from '../../modules/official/fortune';
import { insightsWidgets } from '../../modules/official/insights';
import { useModules } from './ModuleProvider';
import { moduleText } from './text';
class WidgetBoundary extends Component<{ children: ReactNode }, { failed: boolean }> {
  state = { failed: false };
  static getDerivedStateFromError() {
    return { failed: true };
  }
  render() {
    return this.state.failed ? (
      <p role="status">{translate('模块组件暂不可用，其他首页功能仍可使用。')}</p>
    ) : (
      this.props.children
    );
  }
}
export function TodayWidgets({ slot }: { slot: 'today.after-summary' | 'today.sidebar' }) {
  const { state, ready } = useModules();
  return (
    <>
      {ready &&
        [...insightsWidgets, ...fortuneWidgets]
          .filter(
            (w) => w.slot === slot && state.items.some((i) => i.id === w.moduleId && i.enabled),
          )
          .sort((a, b) => a.order - b.order)
          .map((w) => (
            <WidgetBoundary key={w.id}>
              <w.Component />
            </WidgetBoundary>
          ))}
    </>
  );
}
export function ModuleGate({ id, children }: { id: string; children: ReactNode }) {
  const { state, ready } = useModules();
  return !ready ? (
    <p role="status">{translate('正在读取模块配置…')}</p>
  ) : state.items.some((i) => i.id === id && i.enabled) ? (
    children
  ) : (
    <p role="status">{moduleText.disabled}</p>
  );
}
