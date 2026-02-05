<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import StatusIndicator from '$lib/components/StatusIndicator.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { saveApiKey, getApiKey, testConnection } from '$lib/utils/commands';

  let apiKey = $state('');
  let apiKeyLoaded = $state(false);
  let testing = $state(false);
  let testResult = $state<{ success: boolean; message: string } | null>(null);

  $effect(() => {
    loadApiKey();
  });

  async function loadApiKey() {
    try {
      const key = await getApiKey(appState.provider.name);
      apiKey = key ?? '';
      apiKeyLoaded = true;
    } catch (e) {
      console.error('Failed to load API key:', e);
      apiKeyLoaded = true;
    }
  }

  async function handleSave() {
    try {
      if (apiKey) {
        await saveApiKey(appState.provider.name, apiKey);
      }
      appState.provider = { ...appState.provider };
    } catch (e) {
      console.error('Failed to save:', e);
    }
  }

  async function handleTest() {
    testing = true;
    testResult = null;
    try {
      const result = await testConnection(
        appState.provider.base_url,
        apiKey,
        appState.provider.model
      );
      testResult = {
        success: result.success,
        message: result.success
          ? `Connected! Latency: ${result.latency_ms}ms`
          : result.error ?? 'Connection failed',
      };
    } catch (e) {
      testResult = { success: false, message: `Error: ${e}` };
    } finally {
      testing = false;
    }
  }
</script>

<div class="flex flex-col gap-3 p-1">
  <GlassCard padding="p-4">
    <div class="flex flex-col gap-3">
      <label class="flex flex-col gap-1">
        <span class="text-xs text-white/50">Provider Name</span>
        <input
          type="text"
          bind:value={appState.provider.name}
          class="bg-white/10 border border-white/15 rounded-lg px-3 py-2 text-sm text-white/90 outline-none focus:border-white/30"
        />
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs text-white/50">Base URL</span>
        <input
          type="text"
          bind:value={appState.provider.base_url}
          placeholder="https://api.openai.com"
          class="bg-white/10 border border-white/15 rounded-lg px-3 py-2 text-sm text-white/90 font-mono outline-none focus:border-white/30"
        />
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs text-white/50">Model</span>
        <input
          type="text"
          bind:value={appState.provider.model}
          placeholder="gpt-4o-mini"
          class="bg-white/10 border border-white/15 rounded-lg px-3 py-2 text-sm text-white/90 font-mono outline-none focus:border-white/30"
        />
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs text-white/50">API Key</span>
        <input
          type="password"
          bind:value={apiKey}
          placeholder={apiKeyLoaded ? '••••••••' : 'Loading...'}
          class="bg-white/10 border border-white/15 rounded-lg px-3 py-2 text-sm text-white/90 font-mono outline-none focus:border-white/30"
        />
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs text-white/50">Timeout: {appState.provider.timeout_secs}s</span>
        <input
          type="range"
          min="5"
          max="120"
          step="5"
          bind:value={appState.provider.timeout_secs}
          class="w-full accent-blue-400"
        />
      </label>
    </div>
  </GlassCard>

  <div class="flex gap-2">
    <button
      class="flex-1 py-2 rounded-xl text-sm font-medium bg-blue-500/20 hover:bg-blue-500/30 text-blue-200 border border-blue-400/20"
      onclick={handleSave}
    >
      Save
    </button>
    <button
      class="flex-1 py-2 rounded-xl text-sm font-medium bg-white/10 hover:bg-white/15 text-white/70 border border-white/10 disabled:opacity-50"
      onclick={handleTest}
      disabled={testing}
    >
      {testing ? 'Testing...' : 'Test Connection'}
    </button>
  </div>

  {#if testResult}
    <div class="text-center text-xs {testResult.success ? 'text-green-300' : 'text-red-300'}">
      {testResult.message}
    </div>
  {/if}
</div>
