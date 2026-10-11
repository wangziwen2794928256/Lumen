const loaders: Record<string, () => Promise<unknown>> = {
  '/fortune': () => import('../pages/fortune/FortunePage'),
  '/profile': () => import('../pages/profile/ProfilePage'),
  '/journal': () => import('../pages/journal/JournalPage'),
  '/coding': () => import('../pages/coding/CodingPage'),
  '/insights': () => import('../pages/insights/InsightsPage'),
  '/vault': () => import('../pages/vault/VaultPage'),
  '/music': () => import('../pages/music/MusicPage'),
};
export function preloadRoute(route: string) {
  void loaders[route]?.().catch(() => {});
}
