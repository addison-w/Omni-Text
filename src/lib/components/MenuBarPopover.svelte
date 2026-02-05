<script lang="ts">
  import StatusIndicator from './StatusIndicator.svelte';
  import GlassCard from './GlassCard.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { testConnection, getApiKey } from '$lib/utils/commands';

  interface Props {
    onOpenSettings: () => void;
  }

  let { onOpenSettings }: Props = $props();

  let testingConnection = $state(false);
  let connectionResult = $state<string | null>(null);

  async function handleTestConnection() {
    testingConnection = true;
    connectionResult = null;
    try {
      const apiKey = await getApiKey(appState.provider.name) ?? '';
      const result = await testConnection(
        appState.provider.base_url,
        apiKey,
        appState.provider.model
      );
      if (result.success) {
        connectionResult = `Connected (${result.latency_ms}ms)`;
      } else {
        connectionResult = result.error ?? 'Connection failed';
      }
    } catch (e) {
      connectionResult = `Error: ${e}`;
    } finally {
      testingConnection = false;
    }
  }

  function handleQuit() {
    import('@tauri-apps/api/core').then(({ invoke }) => {
      // The tray menu handles quit, but we can also exit from here
      window.close();
    });
  }
</script>

<div class="flex flex-col h-full p-4 gap-3">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <span class="text-sm font-semibold text-white/90">Omni Text</span>
    </div>
    <StatusIndicator status={appState.status} />
  </div>

  <div class="h-px bg-white/10"></div>

  <!-- Toggles -->
  <GlassCard padding="p-3">
    <div class="flex flex-col gap-3">
      <label class="flex items-center justify-between cursor-pointer">
        <span class="text-sm text-white/80">Enabled</span>
        <input
          type="checkbox"
          bind:checked={appState.isEnabled}
          class="w-9 h-5 rounded-full appearance-none cursor-pointer
            bg-white/20 checked:bg-green-500/60
            relative after:content-[''] after:absolute after:top-0.5 after:left-0.5
            after:w-4 after:h-4 after:rounded-full after:bg-white after:shadow-sm
            after:transition-transform checked:after:translate-x-4"
        />
      </label>
      <label class="flex items-center justify-between cursor-pointer">
        <span class="text-sm text-white/80">Private Mode</span>
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

  <!-- Actions summary -->
  <GlassCard padding="p-3">
    <div class="flex flex-col gap-2">
      <span class="text-xs text-white/50 uppercase tracking-wider">Actions</span>
      {#each appState.actions.filter(a => a.enabled) as action}
        <div class="flex items-center justify-between">
          <span class="text-sm text-white/80">{action.name}</span>
          <span class="px-2 py-0.5 rounded text-xs font-mono bg-white/10 text-white/50">
            {action.hotkey
              .replace('CommandOrControl', '⌘')
              .replace('Alt', '⌥')
              .replace('Shift', '⇧')
              .replace('Control', '⌃')
              .replace(/\+/g, '')}
          </span>
        </div>
      {/each}
    </div>
  </GlassCard>

  <!-- Connection Test -->
  <button
    class="w-full py-2 rounded-xl text-sm font-medium transition-all
      bg-white/10 hover:bg-white/15 text-white/80 border border-white/10
      disabled:opacity-50"
    onclick={handleTestConnection}
    disabled={testingConnection}
  >
    {testingConnection ? 'Testing...' : 'Test Connection'}
  </button>
  {#if connectionResult}
    <p class="text-xs text-center {connectionResult.startsWith('Connected') ? 'text-green-300' : 'text-red-300'}">
      {connectionResult}
    </p>
  {/if}

  <div class="flex-1"></div>

  <div class="h-px bg-white/10"></div>

  <!-- Footer buttons -->
  <div class="flex gap-2">
    <button
      class="flex-1 py-2 rounded-xl text-sm font-medium transition-all
        bg-white/10 hover:bg-white/15 text-white/80 border border-white/10"
      onclick={onOpenSettings}
    >
      Settings...
    </button>
    <button
      class="px-4 py-2 rounded-xl text-sm font-medium transition-all
        bg-white/10 hover:bg-red-500/20 text-white/50 hover:text-red-300 border border-white/10"
      onclick={handleQuit}
    >
      Quit
    </button>
  </div>
</div>
