import { Navigate, Route, Routes, useLocation } from 'react-router-dom';
import { lazy, Suspense } from 'react';
import { VaultRoute } from '../modules/official/vault';
import { ProfileRoute } from '../modules/official/profile';
import { DeepLink } from '../kernel/DeepLink';
import { MusicRoute } from '../modules/official/music';
import { InsightsRoute as InsightsPage } from '../modules/official/insights';
import { ModuleProvider } from '../kernel/modules/ModuleProvider';
import { ModuleGate } from '../kernel/modules/Extensions';
const CodingPage = lazy(() =>
  import('../pages/coding/CodingPage').then((m) => ({ default: m.CodingPage })),
);
const JournalPage = lazy(() =>
  import('../pages/journal/JournalPage').then((m) => ({ default: m.JournalPage })),
);
const CapturePage = lazy(() =>
  import('../pages/journal/CapturePage').then((m) => ({ default: m.CapturePage })),
);
import { BootstrapPage } from '../pages/bootstrap/BootstrapPage';
import { AboutPage } from '../pages/about/AboutPage';
import { TodayPage } from '../pages/today/TodayPage';
import { FocusPage } from '../pages/focus/FocusPage';
import { CalendarPage } from '../pages/calendar/CalendarPage';
import { RoadmapPage } from '../pages/roadmap/RoadmapPage';
import { DesignPage } from '../pages/design/DesignPage';
import { SettingsPage } from '../pages/settings/SettingsPage';
import { AppShell } from '../components/layout/AppShell';
import { AppearanceProvider } from './providers/AppearanceProvider';
export function AppRoutes() {
  const location = useLocation();
  if (location.pathname === '/capture')
    return (
      <AppearanceProvider>
        <CapturePage />
      </AppearanceProvider>
    );
  return (
    <AppearanceProvider>
      <ModuleProvider>
        <DeepLink />
        <AppShell>
          <Suspense
            fallback={
              <div className="route-loading" role="status">
                正在载入模块…
              </div>
            }
          >
            <Routes>
              <Route
                path="/vault"
                element={
                  <ModuleGate id="lumen.vault">
                    <VaultRoute />
                  </ModuleGate>
                }
              />
              <Route
                path="/music"
                element={
                  <ModuleGate id="lumen.music">
                    <MusicRoute />
                  </ModuleGate>
                }
              />
              <Route path="/" element={<TodayPage />} />
              <Route
                path="/focus"
                element={
                  <ModuleGate id="lumen.focus">
                    <FocusPage />
                  </ModuleGate>
                }
              />
              <Route
                path="/calendar"
                element={
                  <ModuleGate id="lumen.calendar">
                    <CalendarPage />
                  </ModuleGate>
                }
              />
              <Route
                path="/roadmap"
                element={
                  <ModuleGate id="lumen.roadmap">
                    <RoadmapPage />
                  </ModuleGate>
                }
              />
              <Route
                path="/journal"
                element={
                  <ModuleGate id="lumen.journal">
                    <JournalPage />
                  </ModuleGate>
                }
              />
              <Route
                path="/coding"
                element={
                  <ModuleGate id="lumen.coding">
                    <CodingPage />
                  </ModuleGate>
                }
              />
              <Route
                path="/insights"
                element={
                  <ModuleGate id="lumen.insights">
                    <InsightsPage />
                  </ModuleGate>
                }
              />
              <Route path="/design" element={<DesignPage />} />
              <Route path="/settings" element={<SettingsPage />} />
              <Route path="/bootstrap" element={<BootstrapPage />} />
              <Route path="/about" element={<AboutPage />} />
              <Route
                path="/profile"
                element={
                  <ModuleGate id="lumen.profile">
                    <ProfileRoute />
                  </ModuleGate>
                }
              />
              <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
          </Suspense>
        </AppShell>
      </ModuleProvider>
    </AppearanceProvider>
  );
}
