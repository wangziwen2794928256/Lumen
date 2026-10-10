import type { Facts, Day } from '../../src/features/insights/model';
export function insightFacts(): Facts {
  const d: Day = {
    date: '2026-10-04',
    deep_minutes: 30,
    coding_minutes: 10,
    tasks_completed: 1,
    journal_count: 1,
    problem_count: 1,
    music_count: 2,
    planned_minutes: 60,
    capacity_minutes: 210,
    status: 'normal',
    status_revision: 0,
    intensity: 3,
  };
  return {
    from: d.date,
    until: d.date,
    days: [d],
    heatmap: [d],
    preferences: {
      weights: [1, 1, 10, 10, 10, 1],
      thresholds: [1, 30, 90, 180],
      stop_words: [],
      memories: true,
    },
    preferences_revision: 0,
    words: [
      {
        word: '数学',
        count: 2,
        records: [
          {
            kind: 'journal',
            id: '00000000-0000-4000-8000-000000000001',
            date: d.date,
            title: '隔离数学记录',
          },
        ],
      },
    ],
    subjects: [{ label: '数学', minutes: 30 }],
    journeys: [{ label: '隔离 Journey', total: 2, completed: 1 }],
    music: [{ label: '隔离歌曲', artist: '隔离艺术家', album: '隔离专辑', count: 2, seconds: 180 }],
    topics: [{ label: 'DP', count: 1 }],
    records: [],
    task_summary: { total: 2, completed: 1, unfinished: 1, moved: 0 },
    consistency: {
      days7: { active: 1, eligible: 7, protected: 0 },
      days28: { active: 1, eligible: 28, protected: 0 },
    },
    previous: {
      deep_minutes: 20,
      coding_minutes: 0,
      planned_minutes: 60,
      tasks_completed: 0,
      journals: 0,
    },
  };
}
