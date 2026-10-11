import { translate } from '../../lib/locale';
export const focusText = {
  title: translate('把时间留给一件事。'),
  subtitle: translate('计划是参考，实际进展由本地计时记录。'),
  choose: translate('选择专注任务'),
  empty: translate('先创建一个任务，再开始专注。'),
  planned: translate('计划时长'),
  paused: translate('累计暂停'),
  interrupted: translate('累计中断'),
  blocked: translate('系统仍处于锁定或休眠状态，解除后可手动继续。'),
  discard: translate('放弃本次记录'),
  discardHint: translate('放弃会话后，本次时长不计入实际成果；任务重新安排。'),
  hint: translate('暂停、锁屏、休眠和离线时间不计入专注。恢复后由你选择继续或结束。'),
  reasons: {
    manual: translate('已手动中断。'),
    restart: translate('应用已重新启动。'),
    timestamp_gap: translate('检测到计时中断或系统时间变化。'),
    screen_lock: translate('系统曾锁屏或断开会话。'),
    sleep: translate('系统曾进入休眠。'),
  } as Record<string, string>,
};
