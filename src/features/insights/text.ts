import { translate } from '../../lib/locale';
export const insightsText = {
  title: translate('看见真实记录，留出回顾的空间。'),
  description: translate('本机汇总实际区间、任务、日记、编码与音乐。活动强度只描述记录量。'),
  desktop: translate('请在桌面应用中查看本机数据。'),
  overview: translate('核心洞察'),
  heatmap: translate('年度活动'),
  words: translate('本周词语'),
  music: translate('音乐云'),
  review: translate('周回顾'),
  month: translate('月快照'),
  settings: translate('活动配置'),
  day: translate('当日详情'),
  save: translate('保存'),
  refresh: translate('重新载入'),
  pending: translate('未确认输入已在本机保留，保存成功后清理。'),
  error: translate('读取或保存失败，输入已保留。'),
  empty: translate('暂无记录'),
  saved: translate('已保存'),
  private: translate('仅导出本机私密快照；含统计标签与词语，不含记录正文或个人心得。'),
  definition: translate(
    '深度专注按实际跨日区间。编码含关联专注与额外手工时间，与深度专注有重叠，不相加为总用时。任务按计划日期统计当前状态，非历史完成事件。',
  ),
  highlights: translate('记录摘选（最多 500 条，统计不受此限制）'),
  cloud: translate(
    '词频按用户日记/快速记录/心得计算；不含系统总结。显示前 50 词，每词最多 20 个来源。',
  ),
};
