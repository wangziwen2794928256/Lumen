import { locale } from '../../lib/locale';
const copy = {
  'zh-CN': {
    heading: '每日运势',
    eyebrow: '给今天一点轻盈',
    intro: '一张轻松的每日签，陪你开始今天。把注意力留给自己想做的事。',
    note: '本地每日签，仅供娱乐与灵感，不预测现实结果，也不参与计划安排。',
    privacy:
      '只依据设备本地日期从固定签文中轮换；无需生日、账号或网络。当天重复打开保持一致，跨日自动更新。',
    details: '看看今日签',
    settings: '在模块管理中关闭',
    suggestion: '今日小提示',
    signs: [
      ['从容', '按自己的节奏来。', '选一件想做的小事，给它留一点时间。'],
      ['新意', '换个角度，也许有新的发现。', '试着用不同的方式整理一条笔记。'],
      ['晴朗', '给今天留一点明亮。', '打开窗，或留意身边一件好看的小东西。'],
      ['相遇', '有趣的想法，可能来自一次交流。', '如果愿意，和朋友分享一个最近的发现。'],
      ['专注', '一小段安静，也很珍贵。', '把暂时用不到的窗口收起来。'],
      ['留白', '不必把每个空隙都填满。', '为休息留一段不需要解释的时间。'],
      ['积累', '走过的路，值得被看见。', '翻一翻旧笔记，找一个已经理解的知识点。'],
      ['好奇', '允许自己问一个新问题。', '记下一个想了解的话题，暂时不用找答案。'],
      ['舒展', '生活不只有待办清单。', '活动一下身体，或听一首喜欢的歌。'],
      ['轻装', '少一点，也可以很好。', '看看今天是否有一项可以安心推迟的安排。'],
      ['灵感', '一个不完整的想法也值得记下。', '用一句话留下此刻的灵感。'],
      ['自在', '今天可以由你来定义。', '挑一个适合此刻精力的开始方式。'],
    ],
  },
  en: {
    heading: 'Daily fortune',
    eyebrow: 'A lighter start to today',
    intro: 'A small daily sign to start your day. Leave room for what matters to you.',
    note: 'A local daily sign for entertainment and inspiration. It predicts no outcomes and never changes your plan.',
    privacy:
      'Rotates through a fixed collection using your device’s local date. No birthday, account or network required. The same sign stays all day and refreshes after midnight.',
    details: 'Read today’s sign',
    settings: 'Turn off in module management',
    suggestion: 'A small suggestion',
    signs: [
      ['Ease', 'Let your own pace lead.', 'Choose one small thing and leave a little time for it.'],
      [
        'Freshness',
        'Another angle may reveal something new.',
        'Try a different way to arrange a note.',
      ],
      [
        'Brightness',
        'Leave a little light in the day.',
        'Open a window or notice something beautiful nearby.',
      ],
      [
        'Connection',
        'An exchange can spark an interesting idea.',
        'If you feel like it, share a recent discovery with a friend.',
      ],
      [
        'Focus',
        'A quiet moment can be precious.',
        'Put away the windows you do not need right now.',
      ],
      [
        'Space',
        'Every gap does not need filling.',
        'Leave some time for rest, with no explanation needed.',
      ],
      [
        'Growth',
        'The ground you have covered deserves a glance.',
        'Look through an old note for something you now understand.',
      ],
      [
        'Curiosity',
        'Let yourself ask a new question.',
        'Note a topic to explore; an answer can wait.',
      ],
      [
        'Stretch',
        'There is more to life than a task list.',
        'Move a little, or listen to a song you enjoy.',
      ],
      [
        'Lightness',
        'A little less can feel just right.',
        'See whether something on today’s list can comfortably wait.',
      ],
      [
        'Inspiration',
        'An unfinished thought is worth keeping.',
        'Capture a passing idea in one sentence.',
      ],
      ['Freedom', 'You get to define today.', 'Choose a start that suits the energy you have now.'],
    ],
  },
} as const;
export const fortuneText = copy[locale];
