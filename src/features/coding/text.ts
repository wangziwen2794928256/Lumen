import { translate } from '../../lib/locale';
export const codingText = {
  title: translate('一道题，一次更清晰的思考。'),
  hint: translate('手动记录题目与尝试，留下解法和真实用时。'),
  newProblem: translate('添加题目'),
  newContest: translate('添加竞赛'),
  problems: translate('题目'),
  contests: translate('竞赛'),
  analytics: translate('周统计'),
  save: translate('保存'),
  back: translate('返回'),
  edit: translate('编辑'),
  trash: translate('移入回收站'),
  restore: translate('恢复'),
  noSamples: translate('暂无样本'),
  empty: translate('还没有题目记录。'),
  attempt: translate('记录尝试'),
  attempts: translate('尝试记录'),
  error: translate('操作未保存，输入已保留，请检查后重试。'),
  desktop: translate('请在桌面程序中使用，浏览器没有个人数据。'),
  conflict: translate('记录已变化，请重新载入后再保存；输入已保留。'),
  duplicate: translate('该平台与题号已存在，请打开原题增加尝试。'),
  focusUsed: translate('这段专注已关联另一条尝试，不能重复计时。'),
  pending: translate('发现未提交的 Coding 输入，可恢复继续编辑。'),
  recover: translate('恢复未提交输入'),
  difficulty: {
    unknown: translate('未标注'),
    easy: translate('简单'),
    medium: translate('中等'),
    hard: translate('困难'),
  },
  outcome: {
    accepted: translate('通过'),
    wrong_answer: translate('答案错误'),
    time_limit: translate('超时'),
    runtime_error: translate('运行错误'),
    compile_error: translate('编译错误'),
    unfinished: translate('未完成'),
  },
};
export function codingError(e: unknown) {
  const code =
    e && typeof e === 'object' && 'code' in e
      ? String(e.code)
      : e instanceof Error
        ? e.message
        : '';
  return code === 'DESKTOP_REQUIRED'
    ? codingText.desktop
    : code === 'CODING_CONFLICT'
      ? codingText.conflict
      : code === 'CODING_DUPLICATE'
        ? codingText.duplicate
        : code === 'CODING_FOCUS_USED'
          ? codingText.focusUsed
          : codingText.error;
}
