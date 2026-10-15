import { flushSync } from 'svelte';
import { describe, expect, it } from 'vitest';
import { all, render } from '../test/render';
import Text from './Text.svelte';

describe('Text', () => {
  it('sets the product name apart and keeps the rest as plain text', () => {
    const { target } = render(Text, { value: 'Everything deadloader did, next to deadloader.exe' });
    expect(target.textContent).toBe('Everything deadloader did, next to deadloader.exe');
    expect(all(target, '.dl-brand').map((span) => span.textContent)).toEqual(['deadloader']);
    expect(target.children).toHaveLength(1);
  });

  it('follows a new value', () => {
    const props = $state({ value: 'deadloader' });
    const { target } = render(Text, props);
    props.value = 'Nothing yet';
    flushSync();
    expect(target.textContent).toBe('Nothing yet');
    expect(target.querySelector('.dl-brand')).toBeNull();
  });

  it('renders nothing without a value', () => {
    const { target } = render(Text, { value: undefined });
    expect(target.textContent).toBe('');
  });
});
