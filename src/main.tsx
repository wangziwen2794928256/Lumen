import { locale } from './lib/locale';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './app/App';
import './styles/base.css';
import { readAppearance } from './lib/appearance';
document.documentElement.lang = locale;
const preference = readAppearance();
document.documentElement.dataset.theme =
  preference.mode === 'system'
    ? window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light'
    : preference.mode;
document.documentElement.dataset.accent = preference.accent;
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
