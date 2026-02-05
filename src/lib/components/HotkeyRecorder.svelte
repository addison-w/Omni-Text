<script lang="ts">
  interface Props {
    value: string;
    onchange?: (shortcut: string) => void;
  }

  let { value, onchange }: Props = $props();

  let recording = $state(false);
  let displayValue = $derived(formatShortcut(value));

  function formatShortcut(shortcut: string): string {
    if (!shortcut) return 'Click to record';
    return shortcut
      .replace('CommandOrControl', '⌘')
      .replace('Alt', '⌥')
      .replace('Shift', '⇧')
      .replace('Control', '⌃')
      .replace(/\+/g, '');
  }

  function startRecording() {
    recording = true;
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    const parts: string[] = [];
    if (e.metaKey) parts.push('CommandOrControl');
    if (e.ctrlKey && !e.metaKey) parts.push('Control');
    if (e.altKey) parts.push('Alt');
    if (e.shiftKey) parts.push('Shift');

    // Only finalize on a non-modifier key
    const key = e.key;
    if (!['Meta', 'Control', 'Alt', 'Shift'].includes(key)) {
      const keyName = key.length === 1 ? key.toUpperCase() : key;
      parts.push(keyName);
      const shortcut = parts.join('+');
      recording = false;
      onchange?.(shortcut);
    }
  }
</script>

<button
  class="px-3 py-1.5 rounded-lg text-xs font-mono transition-all
    {recording
      ? 'bg-blue-500/30 border-blue-400/50 text-blue-200 animate-pulse'
      : 'bg-white/10 border border-white/20 text-white/70 hover:bg-white/15'}"
  onclick={startRecording}
  onkeydown={handleKeyDown}
>
  {recording ? 'Press shortcut...' : displayValue}
</button>
