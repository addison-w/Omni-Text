<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { checkAccessibilityPermission, requestAccessibilityPermission } from '$lib/utils/commands';

  let accessibilityGranted = $state(false);

  $effect(() => {
    checkAccessibilityPermission().then(granted => {
      accessibilityGranted = granted;
    });
  });
</script>

<div class="flex flex-col gap-3 p-1">
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <span class="text-xs text-white/50 uppercase tracking-wider">Accessibility</span>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full {accessibilityGranted ? 'bg-green-400' : 'bg-red-400'}"></div>
            <span class="text-sm text-white/80">
              {accessibilityGranted ? 'Permission Granted' : 'Permission Required'}
            </span>
          </div>
          {#if !accessibilityGranted}
            <button
              class="px-3 py-1 rounded-lg text-xs bg-blue-500/20 hover:bg-blue-500/30 text-blue-200"
              onclick={requestAccessibilityPermission}
            >
              Grant Access
            </button>
          {/if}
        </div>
        <p class="text-xs text-white/40">
          Required to read and replace selected text. Open System Settings → Privacy & Security → Accessibility.
        </p>
      </div>
    </div>
  </GlassCard>

  <GlassCard padding="p-4">
    <div class="flex flex-col gap-3">
      <span class="text-xs text-white/50 uppercase tracking-wider">About</span>
      <div class="flex items-center justify-between text-sm text-white/60">
        <span>Version</span>
        <span class="font-mono">0.1.0</span>
      </div>
      <p class="text-xs text-white/30">
        Omni Text — macOS hotkey text rewrite app.
        Press ⌘Z to undo any rewrite.
      </p>
    </div>
  </GlassCard>
</div>
