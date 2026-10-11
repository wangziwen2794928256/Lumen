import { translate } from '../../lib/locale';
export const moduleText = {
  heading: translate('模块管理'),
  hint: translate(
    '启用控制页面和模块入口；隐藏只改变侧栏。置顶模块优先显示。拖动排列，或用上下按钮。关闭模块不会删除记录。',
  ),
  enabled: translate('启用'),
  hidden: translate('隐藏'),
  pinned: translate('置顶'),
  up: translate('上移'),
  down: translate('下移'),
  save: translate('保存模块配置'),
  error: translate('保存失败或版本已变化；当前选择已保留，请重新载入后重试。'),
  reload: translate('重新载入模块配置'),
  disabled: translate('此模块已停用，可在设置中重新启用。'),
  saved: translate('模块配置已保存'),
  required: translate('核心模块不能停用；安全系统始终属于 Kernel。'),
};
