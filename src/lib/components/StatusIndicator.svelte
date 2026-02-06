<script lang="ts">
  interface Props {
    status: 'ready' | 'processing' | 'not-ready';
  }

  let { status }: Props = $props();

  const config = $derived({
    ready: { color: 'bg-green-400', label: 'Ready' },
    processing: { color: 'bg-orange-400 animate-pulse', label: 'Processing' },
    'not-ready': { color: 'bg-red-400', label: 'Not Ready' },
  }[status]);
</script>

<div class="flex items-center gap-1.5">
  <div class="w-2 h-2 rounded-full {config.color}"></div>
  <span class="text-xs text-black/50 dark:text-white/50">{config.label}</span>
</div>
