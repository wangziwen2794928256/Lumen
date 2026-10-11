import { locale, translate } from '../../lib/locale';
import { useEffect, useState, type ReactNode } from 'react';
import { NavLink, useLocation } from 'react-router-dom';
import {
  Sun,
  Timer,
  CalendarDays,
  Map,
  NotebookPen,
  ChartNoAxesCombined,
  Archive,
  UserRound,
  Settings,
  PanelLeftClose,
  PanelLeftOpen,
  Layers,
  Leaf,
  CodeXml,
  Music2,
  Sparkles,
} from 'lucide-react';
import { useAppearance } from '../../app/providers/appearance-context';
import { Button } from '../ui/button';
import { designText as d } from '../../lib/design-i18n';
import { t } from '../../lib/i18n';
import { useModules } from '../../kernel/modules/ModuleProvider';
import { ordered, reorder } from '../../kernel/modules/registry';
import { preloadRoute } from '../../app/preload';
import { Onboarding } from '../../features/desktop/Onboarding';
const icons = {
  Sun,
  Timer,
  CalendarDays,
  Map,
  NotebookPen,
  ChartNoAxesCombined,
  Archive,
  UserRound,
  CodeXml,
  Music2,
  Sparkles,
};
export function AppShell({ children }: { children: ReactNode }) {
  const { value, update } = useAppearance();
  const { pathname } = useLocation();
  const modules = useModules();
  const [drag, setDrag] = useState(''),
    [moduleError, setModuleError] = useState('');
  const items = ordered(modules.state, true);
  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);
  return (
    <div className={'app-shell ' + (value.collapsed ? 'sidebar-collapsed' : '')}>
      <a
        href="#main-content"
        className="skip-link"
        onClick={(event) => {
          event.preventDefault();
          const content = document.getElementById('main-content');
          content?.focus();
          content?.scrollIntoView({ block: 'start' });
        }}
      >
        {d.skip}
      </a>
      <aside className="sidebar">
        <div className="brand">
          <Leaf size={23} aria-hidden="true" />
          <span>Lumen</span>
        </div>
        <Button
          variant="ghost"
          size="icon"
          className="collapse-button"
          aria-label={value.collapsed ? d.expand : d.collapse}
          aria-expanded={!value.collapsed}
          onClick={() => update({ collapsed: !value.collapsed })}
        >
          {value.collapsed ? <PanelLeftOpen size={19} /> : <PanelLeftClose size={19} />}
        </Button>
        <nav aria-label={translate('主要导航')}>
          {items.map((module, i) => {
            const url = module.route,
              label = module.name[locale],
              Icon = icons[module.icon as keyof typeof icons] ?? Layers;
            return (
              <NavLink
                key={module.id}
                draggable
                onDragStart={(e) => {
                  e.dataTransfer.setData('text/plain', module.id);
                  setDrag(module.id);
                }}
                onDragOver={(e) => e.preventDefault()}
                onDrop={(e) => {
                  e.preventDefault();
                  if (e.dataTransfer.getData('text/plain') === drag)
                    void modules
                      .save(reorder(modules.state, drag, module.id))
                      .catch(() => setModuleError(translate('模块排列未保存，请在设置重试。')));
                  setDrag('');
                }}
                to={url}
                onPointerEnter={() => preloadRoute(url)}
                onFocus={() => preloadRoute(url)}
                end={url === '/'}
                title={value.collapsed ? label : undefined}
                aria-label={label}
                className={i === 3 ? 'nav-section' : ''}
              >
                <Icon size={19} aria-hidden="true" />
                <span>{label}</span>
              </NavLink>
            );
          })}
          {(moduleError || modules.error) && <p role="alert">{moduleError || modules.error}</p>}
        </nav>
        <div className="sidebar-bottom">
          <NavLink
            to="/design"
            aria-label={d.library}
            title={value.collapsed ? d.library : undefined}
          >
            <Layers size={19} aria-hidden="true" />
            <span>{d.library}</span>
          </NavLink>
          <NavLink
            to="/settings"
            aria-label={d.settings}
            title={value.collapsed ? d.settings : undefined}
          >
            <Settings size={19} aria-hidden="true" />
            <span>{d.settings}</span>
          </NavLink>
          <p>
            <span className="local-dot" />
            <span>{d.local}</span>
          </p>
        </div>
      </aside>
      <div className="workspace">
        <Onboarding />
        <header className="topbar">
          <span className="topbar-label">PERSONAL GROWTH OS</span>
          <div className="topbar-links">
            <NavLink to="/bootstrap">{t.home}</NavLink>
            <NavLink to="/about">{t.about}</NavLink>
          </div>
        </header>
        <main id="main-content" tabIndex={-1}>
          <div className="route-content" key={pathname}>
            {children}
          </div>
        </main>
        <footer>{d.phase}</footer>
      </div>
    </div>
  );
}
