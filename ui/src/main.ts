import { mount } from 'svelte';
import App from './App.svelte';
import './theme/tokens.css';
import './app.css';

const target = document.getElementById('app');
if (!target) {
  throw new Error('the #app element is missing from index.html');
}

export default mount(App, { target });
