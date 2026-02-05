<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    padding?: string;
    rounded?: string;
    children: Snippet;
  }

  let { padding = 'p-4', rounded = 'rounded-2xl', children }: Props = $props();
</script>

<div class="bg-white/10 dark:bg-white/5 border border-white/20 dark:border-white/10 shadow-xl backdrop-blur-sm {rounded} {padding}">
  {@render children()}
</div>
