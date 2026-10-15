import { mount } from 'svelte';
import App from './App.svelte';
import { api } from './lib/api';
import { applyLocale } from './lib/locale';
import './theme/tokens.css';
import './theme/components.css';
import './i18n/fonts.css';
import './app.css';

const target = document.getElementById('app');
if (!target) {
  throw new Error('the #app element is missing from index.html');
}

// The texts and the clock are right from the first frame the app draws.
export default applyLocale(api).then(() => mount(App, { target }));
