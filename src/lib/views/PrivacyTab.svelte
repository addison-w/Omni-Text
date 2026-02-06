<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import Database from '@tauri-apps/plugin-sql';

  let confirmingClear = $state(false);

  async function clearAllHistory() {
    try {
      const db = await Database.load('sqlite:omni_text_history.db');
      await db.execute('DELETE FROM history');
      confirmingClear = false;
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }
</script>

<div class="flex flex-col gap-3 p-1">
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-4">
      <label class="flex items-center justify-between cursor-pointer">
        <div class="flex flex-col">
          <span class="text-sm text-white/80">Private Mode</span>
          <span class="text-xs text-white/40">When enabled, new rewrites won't be saved to history</span>
        </div>
        <input
          type="checkbox"
          bind:checked={appState.privacyMode}
          class="w-9 h-5 rounded-full appearance-none cursor-pointer
            bg-white/20 checked:bg-blue-500/60
            relative after:content-[''] after:absolute after:top-0.5 after:left-0.5
            after:w-4 after:h-4 after:rounded-full after:bg-white after:shadow-sm
            after:transition-transform checked:after:translate-x-4"
        />
      </label>
    </div>
  </GlassCard>

  <GlassCard padding="p-4">
    <div class="flex flex-col gap-2">
      <span class="text-xs text-white/50 uppercase tracking-wider">Data Disclosure</span>
      <p class="text-sm text-white/60">
        Selected text is sent to <span class="text-white/80 font-medium">{appState.provider.name}</span> at
        <span class="font-mono text-white/70">{appState.provider.base_url}</span> for processing.
      </p>
      <p class="text-sm text-white/60">
        No data is sent to Omni Text servers. All processing happens via your configured provider.
      </p>
    </div>
  </GlassCard>

  <GlassCard padding="p-4">
    <div class="flex flex-col gap-3">
      <span class="text-xs text-white/50 uppercase tracking-wider">Data Management</span>
      {#if confirmingClear}
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 rounded-xl text-sm font-medium bg-red-500/20 hover:bg-red-500/30 text-red-300 border border-red-400/20"
            onclick={clearAllHistory}
          >
            Confirm Clear
          </button>
          <button
            class="flex-1 py-2 rounded-xl text-sm font-medium bg-white/10 hover:bg-white/15 text-white/70 border border-white/10"
            onclick={() => confirmingClear = false}
          >
            Cancel
          </button>
        </div>
      {:else}
        <button
          class="w-full py-2 rounded-xl text-sm font-medium bg-red-500/10 hover:bg-red-500/20 text-red-300/70 border border-red-400/10"
          onclick={() => confirmingClear = true}
        >
          Clear All History
        </button>
      {/if}
    </div>
  </GlassCard>
</div>
