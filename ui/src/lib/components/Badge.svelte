<!-- Badge (design: Badge.md): a short status word with an icon. Not interactive. -->
<script lang="ts">
  import type { IconComponent } from './types';

  export type BadgeVariant = 'neutral' | 'accent' | 'info' | 'warning' | 'danger' | 'success' | 'outline';

  interface Props {
    /** Every badge carries an icon and a word: colour never carries the meaning alone. */
    icon: IconComponent;
    label: string;
    variant?: BadgeVariant;
  }

  const { icon: Icon, label, variant = 'neutral' }: Props = $props();
</script>

<span class={['dl-badge', variant !== 'neutral' && `dl-badge--${variant}`]}><Icon class="dl-icon" />{label}</span>
