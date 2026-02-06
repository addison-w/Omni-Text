<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    padding?: string;
    rounded?: string;
    children: Snippet;
  }

  let { padding = 'p-4', rounded = 'rounded-2xl', children }: Props = $props();
</script>

<div class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/20 shadow-xl backdrop-blur-sm {rounded} {padding}">
  {@render children()}
</div>
