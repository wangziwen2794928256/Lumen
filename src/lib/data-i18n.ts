import { translate } from './locale';
export const dataText = {
  status: translate('数据库检查状态'),
  heading: translate('本地数据检查'),
  hint: translate('数据库在这台设备的应用数据目录中。此检查不会创建业务记录。'),
  check: translate('检查本地数据库'),
  busy: translate('正在检查…'),
  ready: translate('数据库已就绪 · 完整性正常'),
  schema: translate('结构版本'),
  records: translate('记录数量'),
  task: translate('任务'),
  journal: translate('日记'),
  browser: translate('仅桌面环境可读取本地数据库。'),
  error: translate('数据库暂不可用。请保留原数据，检查磁盘权限或使用已验证备份。'),
};
