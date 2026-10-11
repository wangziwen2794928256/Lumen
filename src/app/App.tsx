import { translate } from '../lib/locale';
import { HashRouter } from 'react-router-dom';
import { Suspense } from 'react';
import { AppRoutes } from './router';
export function App() {
  return (
    <HashRouter>
      <Suspense fallback={<p role="status">{translate('正在载入…')}</p>}>
        <AppRoutes />
      </Suspense>
    </HashRouter>
  );
}
