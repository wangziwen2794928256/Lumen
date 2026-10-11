import { translate } from './locale';
export const messages = {
  'zh-CN': {
    product: 'Lumen',
    connectionStatus: translate('桌面连接状态'),
    tagline: translate('为长期成长，留一盏灯。'),
    bootstrap: translate('基础工程已就绪'),
    description: translate('这是 Lumen 的基础桌面窗口。业务模块将在后续阶段逐步建立。'),
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
      'React + TypeScript + Tauri 2。设计系统和本地数据库核心已建立，业务功能将在后续阶段开放。',
    ),
  },
};
export const t = messages['zh-CN'];
