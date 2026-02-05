<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import type { HistoryEntry } from '$lib/utils/commands';
  import Database from '@tauri-apps/plugin-sql';

  let entries = $state<HistoryEntry[]>([]);
  let searchQuery = $state('');
  let expandedId = $state<string | null>(null);
  let loading = $state(true);

  $effect(() => {
    loadHistory();
  });

  async function loadHistory() {
    try {
      const db = await Database.load('sqlite:omni_text_history.db');
      const query = searchQuery
        ? `SELECT * FROM history WHERE original_text LIKE '%' || $1 || '%' OR result_text LIKE '%' || $1 || '%' OR action_name LIKE '%' || $1 || '%' ORDER BY timestamp DESC LIMIT 100`
        : 'SELECT * FROM history ORDER BY timestamp DESC LIMIT 100';
      const params = searchQuery ? [searchQuery] : [];
      entries = await db.select<HistoryEntry[]>(query, params);
    } catch (e) {
      console.error('Failed to load history:', e);
    } finally {
      loading = false;
    }
  }

  async function deleteEntry(id: string) {
    try {
      const db = await Database.load('sqlite:omni_text_history.db');
      await db.execute('DELETE FROM history WHERE id = $1', [id]);
      entries = entries.filter(e => e.id !== id);
      if (expandedId === id) expandedId = null;
    } catch (e) {
      console.error('Failed to delete entry:', e);
    }
  }

  async function clearAll() {
    if (!confirm('Clear all history?')) return;
    try {
      const db = await Database.load('sqlite:omni_text_history.db');
      await db.execute('DELETE FROM history');
      entries = [];
      expandedId = null;
    } catch (e) {
      console.error('Failed to clear history:', e);
    }
  }

  function formatDate(timestamp: string): string {
    const d = new Date(timestamp);
    return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  }

  function truncate(text: string, max: number): string {
    return text.length > max ? text.slice(0, max) + '...' : text;
  }

  // Debounced search
  let searchTimeout: ReturnType<typeof setTimeout>;
  function handleSearch(e: Event) {
    searchQuery = (e.target as HTMLInputElement).value;
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => loadHistory(), 300);
  }
</script>

<div class="flex flex-col gap-3 h-full p-1">
  <!-- Search -->
  <div class="relative">
    <input
      type="text"
      placeholder="Search history..."
      value={searchQuery}
      oninput={handleSearch}
      class="w-full bg-white/10 border border-white/15 rounded-lg px-3 py-2 text-sm text-white/90 outline-none focus:border-white/30 pl-8"
    />
    <span class="absolute left-2.5 top-2.5 text-white/30 text-sm">⌕</span>
  </div>

  <!-- Entries -->
  <div class="flex-1 overflow-y-auto flex flex-col gap-2">
    {#if loading}
      <p class="text-sm text-white/40 text-center py-4">Loading...</p>
    {:else if entries.length === 0}
      <p class="text-sm text-white/40 text-center py-4">No history yet</p>
    {:else}
      {#each entries as entry (entry.id)}
        <GlassCard padding="p-3">
          <button
            class="w-full text-left"
            onclick={() => expandedId = expandedId === entry.id ? null : entry.id}
          >
            <div class="flex items-center justify-between">
              <span class="text-xs text-white/50">{formatDate(entry.timestamp)}</span>
              <span class="text-xs text-white/40">{entry.action_name}</span>
            </div>
            <p class="text-sm text-white/80 mt-1">{truncate(entry.original_text, 80)}</p>
          </button>

          {#if expandedId === entry.id}
            <div class="mt-3 pt-3 border-t border-white/10 flex flex-col gap-2">
              <div>
                <span class="text-xs text-white/40">Original</span>
                <p class="text-sm text-white/70 mt-0.5 select-text">{entry.original_text}</p>
              </div>
              <div>
                <span class="text-xs text-white/40">Result</span>
                <p class="text-sm text-white/90 mt-0.5 select-text">{entry.result_text}</p>
              </div>
              <div class="flex items-center justify-between text-xs text-white/30">
                <span>{entry.provider} / {entry.model}</span>
                <span>{entry.duration_ms}ms</span>
              </div>
              <button
                class="mt-1 px-3 py-1 rounded text-xs bg-red-500/20 hover:bg-red-500/30 text-red-300 self-end"
                onclick={() => deleteEntry(entry.id)}
              >Delete</button>
            </div>
          {/if}
        </GlassCard>
      {/each}
    {/if}
  </div>

  {#if entries.length > 0}
    <button
      class="w-full py-2 rounded-xl text-sm font-medium bg-red-500/10 hover:bg-red-500/20 text-red-300/70 border border-red-400/10"
      onclick={clearAll}
    >
      Clear All History
    </button>
  {/if}
</div>
