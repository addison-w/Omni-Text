<script lang="ts">
  import { fly } from 'svelte/transition';

  interface Props {
    message: string;
    variant?: 'success' | 'error' | 'info';
    visible?: boolean;
  }

  let { message, variant = 'info', visible = true }: Props = $props();

  const icons: Record<string, string> = {
    success: '✓',
    error: '✕',
    info: 'ℹ',
  };

  const colors: Record<string, string> = {
    success: 'bg-green-500/20 border-green-400/30 text-green-200',
    error: 'bg-red-500/20 border-red-400/30 text-red-200',
    info: 'bg-blue-500/20 border-blue-400/30 text-blue-200',
  };
</script>

{#if visible}
  <div
    class="fixed top-3 left-1/2 -translate-x-1/2 z-50 px-4 py-2 rounded-full border backdrop-blur-md text-sm font-medium flex items-center gap-2 {colors[variant]}"
    transition:fly={{ y: -20, duration: 200 }}
  >
    <span class="text-base">{icons[variant]}</span>
    <span>{message}</span>
  </div>
{/if}
