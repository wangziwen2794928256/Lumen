import { translate } from './locale';
export const messages = {
  'zh-CN': {
    product: 'Lumen',
    connectionStatus: translate('桌面连接状态'),
    tagline: translate('为长期成长，留一盏灯。'),
    bootstrap: translate('基础工程已就绪'),
    description: translate(
      '这里检查桌面连接、本地数据库与备份状态。日常记录请使用侧栏中的功能模块。',
    ),
    home: translate('基础窗口'),
    about: translate('关于工程'),
    verify: translate('验证桌面连接'),
    checking: translate('正在验证…'),
    connected: translate('Rust 桌面连接正常'),
    browser: translate('当前为浏览器预览，桌面连接需在 Lumen 窗口中验证。'),
    error: translate('桌面连接失败，请查看本地诊断。'),
    phase: 'Phase 01 · Bootstrap',
    aboutTitle: translate('本地优先，长期可读'),
    aboutDescription: translate(
      'Lumen 是开源的本地成长工作台，支持任务、专注、日历、日记与可选模块。数据保存在本机；社区插件运行时、AI Planner 与移动端仍是未来方向。',
    ),
  },
};
export const t = messages['zh-CN'];
