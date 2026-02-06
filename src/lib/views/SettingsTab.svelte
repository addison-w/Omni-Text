<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { checkAccessibilityPermission, requestAccessibilityPermission } from '$lib/utils/commands';
  import Database from '@tauri-apps/plugin-sql';

  let accessibilityGranted = $state(false);
  let confirmingClear = $state(false);

  $effect(() => {
    checkAccessibilityPermission().then(granted => {
      accessibilityGranted = granted;
    });
  });

  async function clearAllHistory() {
    try {
      const db = await Database.load('sqlite:omni_text_history.db');
      await db.execute('DELETE FROM history');
      confirmingClear = false;
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }

  function handleQuit() {
    import('@tauri-apps/api/core').then(({ invoke }) => {
      invoke('quit_app');
    });
  }
</script>

<div class="flex flex-col gap-3 p-1">
  <!-- Privacy -->
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-4">
      <label class="flex items-center justify-between cursor-pointer">
        <div class="flex flex-col">
          <span class="text-sm text-black/70 dark:text-white/80">Private Mode</span>
          <span class="text-xs text-black/40 dark:text-white/40">New rewrites won't be saved to history</span>
        </div>
        <input
          type="checkbox"
          bind:checked={appState.privacyMode}
          class="w-9 h-5 rounded-full appearance-none cursor-pointer
            bg-black/15 dark:bg-white/20 checked:bg-blue-500/60
            relative after:content-[''] after:absolute after:top-0.5 after:left-0.5
            after:w-4 after:h-4 after:rounded-full after:bg-white after:shadow-sm
            after:transition-transform checked:after:translate-x-4"
        />
      </label>
    </div>
  </GlassCard>

  <!-- Accessibility -->
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-1">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full {accessibilityGranted ? 'bg-green-400' : 'bg-red-400'}"></div>
          <span class="text-sm text-black/70 dark:text-white/80">
            {accessibilityGranted ? 'Accessibility Granted' : 'Accessibility Required'}
          </span>
        </div>
        {#if !accessibilityGranted}
          <button
            class="px-3 py-1 rounded-lg text-xs bg-blue-500/20 hover:bg-blue-500/30 text-blue-700 dark:text-blue-200 cursor-pointer"
            onclick={requestAccessibilityPermission}
          >
            Grant Access
          </button>
        {/if}
      </div>
      <p class="text-xs text-black/40 dark:text-white/40">
        System Settings → Privacy & Security → Accessibility
      </p>
    </div>
  </GlassCard>

  <!-- Data Management -->
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-3">
      <span class="text-xs text-black/45 dark:text-white/50 uppercase tracking-wider">Data</span>
      <p class="text-xs text-black/40 dark:text-white/40">
        Text is sent to <span class="text-black/60 dark:text-white/60 font-medium">{appState.provider.name}</span> at
        <span class="font-mono text-black/50 dark:text-white/50">{appState.provider.base_url}</span> for processing.
        No data is sent to Omni Text servers.
      </p>
      {#if confirmingClear}
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 rounded-xl text-sm font-medium bg-red-500/20 hover:bg-red-500/30 text-red-600 dark:text-red-300 border border-red-400/20 cursor-pointer"
            onclick={clearAllHistory}
          >
            Confirm Clear
          </button>
          <button
            class="flex-1 py-2 rounded-xl text-sm font-medium bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/15 text-black/60 dark:text-white/70 border border-black/10 dark:border-white/10 cursor-pointer"
            onclick={() => confirmingClear = false}
          >
            Cancel
          </button>
        </div>
      {:else}
        <button
          class="w-full py-2 rounded-xl text-sm font-medium bg-red-500/10 hover:bg-red-500/20 text-red-500/70 dark:text-red-300/70 border border-red-400/10 cursor-pointer"
          onclick={() => confirmingClear = true}
        >
          Clear All History
        </button>
      {/if}
    </div>
  </GlassCard>

  <!-- About & Quit -->
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-3">
      <div class="flex items-center justify-between text-sm text-black/50 dark:text-white/60">
        <span>Omni Text</span>
        <span class="font-mono text-black/40 dark:text-white/40">v0.1.0</span>
      </div>
      <button
        class="w-full py-2 rounded-xl text-sm font-medium bg-black/5 dark:bg-white/10 hover:bg-red-500/20 text-black/45 dark:text-white/50 hover:text-red-600 dark:hover:text-red-300 border border-black/10 dark:border-white/10 cursor-pointer transition-colors"
        onclick={handleQuit}
      >
        Quit Omni Text
      </button>
    </div>
  </GlassCard>
</div>
