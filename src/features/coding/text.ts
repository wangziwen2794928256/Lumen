export const codingText = {
  title: '一道题，一次更清晰的思考。',
  hint: '手动记录题目与尝试，留下解法和真实用时。',
  newProblem: '添加题目',
  newContest: '添加竞赛',
  problems: '题目',
  contests: '竞赛',
  analytics: '周统计',
  save: '保存',
  back: '返回',
  edit: '编辑',
  trash: '移入回收站',
  restore: '恢复',
  noSamples: '暂无样本',
  empty: '还没有题目记录。',
  attempt: '记录尝试',
  attempts: '尝试记录',
  error: '操作未保存，输入已保留，请检查后重试。',
  desktop: '请在桌面程序中使用，浏览器没有个人数据。',
  conflict: '记录已变化，请重新载入后再保存；输入已保留。',
  duplicate: '该平台与题号已存在，请打开原题增加尝试。',
  focusUsed: '这段专注已关联另一条尝试，不能重复计时。',
  pending: '发现未提交的 Coding 输入，可恢复继续编辑。',
  recover: '恢复未提交输入',
  difficulty: { unknown: '未标注', easy: '简单', medium: '中等', hard: '困难' },
  outcome: {
    accepted: '通过',
    wrong_answer: '答案错误',
    time_limit: '超时',
    runtime_error: '运行错误',
    compile_error: '编译错误',
    unfinished: '未完成',
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
