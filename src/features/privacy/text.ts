import { translate } from '../../lib/locale';
export const privacyText = {
  backup: translate('本地备份'),
  enabled: translate('启用每日备份'),
  hint: translate(
    '应用运行时每 5 分钟检查；每日只创建一个有效快照。备份包含数据库、设置与附件（含回收站），仅保存在本机。',
  ),
  retention: translate(
    '按最新的日、周、月分桶保留，重叠快照只保留一份。只有新备份校验成功后才清理过期日常备份；手动、迁移和异常备份不自动清理。',
  ),
  daily: translate('每日保留份数（1–90）'),
  weekly: translate('每周保留份数（1–52）'),
  monthly: translate('每月保留份数（1–60）'),
  save: translate('保存备份策略'),
  create: translate('立即创建完整备份'),
  saved: translate('备份策略已保存。'),
  created: translate('完整备份已校验并保存：'),
  error: translate('备份操作失败。原始数据未被替换；请检查磁盘空间、目录权限与附件完整性。'),
  browser: translate('备份需要桌面应用；浏览器模式不会访问本机数据。'),
  pending: translate('尚无本次会话的每日备份成功记录。'),
  success: translate('最近每日备份：'),
  private: translate('备份包含私人内容，请妥善保管。本页不会上传、发布或替换数据库。'),
};
