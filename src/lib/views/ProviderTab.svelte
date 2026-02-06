<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { saveApiKey, getApiKey, deleteApiKey, testConnection } from '$lib/utils/commands';
  import type { ProviderConfig } from '$lib/utils/commands';

  let expandedId = $state<string | null>(null);
  let apiKeys = $state<Record<string, string>>({});
  let apiKeyLoaded = $state<Record<string, boolean>>({});
  let testing = $state<string | null>(null);
  let testResult = $state<{ success: boolean; message: string } | null>(null);
  let confirmingDeleteId = $state<string | null>(null);

  // Load API key when a provider is expanded
  async function loadApiKey(provider: ProviderConfig) {
    if (apiKeyLoaded[provider.id]) return;
    try {
      const key = await getApiKey(provider.name);
      apiKeys[provider.id] = key ?? '';
      apiKeyLoaded[provider.id] = true;
    } catch (e) {
      console.error('Failed to load API key:', e);
      apiKeyLoaded[provider.id] = true;
    }
  }

  function toggleExpand(id: string) {
    confirmingDeleteId = null;
    if (expandedId === id) {
      expandedId = null;
    } else {
      expandedId = id;
      const provider = appState.providers.find(p => p.id === id);
      if (provider) loadApiKey(provider);
    }
  }

  function setActive(id: string) {
    appState.activeProviderId = id;
  }

  function addProvider() {
    const id = `provider-${Date.now()}`;
    const newProvider: ProviderConfig = {
      id,
      name: 'New Provider',
      base_url: 'https://api.openai.com',
      model: 'gpt-4o-mini',
      timeout_secs: 30,
    };
    appState.providers = [...appState.providers, newProvider];
    expandedId = id;
    apiKeyLoaded[id] = true;
    apiKeys[id] = '';
  }

  async function deleteProvider(id: string) {
    const provider = appState.providers.find(p => p.id === id);
    if (provider) {
      deleteApiKey(provider.name).catch(() => {});
    }
    appState.providers = appState.providers.filter(p => p.id !== id);
    if (expandedId === id) expandedId = null;
    // If we deleted the active provider, activate the first remaining one
    if (appState.activeProviderId === id && appState.providers.length > 0) {
      appState.activeProviderId = appState.providers[0].id;
    }
  }

  // Debounced save for provider fields
  let saveTimers: Record<string, ReturnType<typeof setTimeout>> = {};
  function updateProvider(id: string, field: keyof ProviderConfig, value: string | number) {
    const idx = appState.providers.findIndex(p => p.id === id);
    if (idx < 0) return;
    const updated = [...appState.providers];
    updated[idx] = { ...updated[idx], [field]: value };
    appState.providers = updated;
  }

  // Debounced API key save
  let keyTimers: Record<string, ReturnType<typeof setTimeout>> = {};
  function handleApiKeyInput(id: string, value: string) {
    apiKeys[id] = value;
    const provider = appState.providers.find(p => p.id === id);
    if (!provider) return;

    if (keyTimers[id]) clearTimeout(keyTimers[id]);
    keyTimers[id] = setTimeout(async () => {
      if (value) {
        try {
          await saveApiKey(provider.name, value);
        } catch (e) {
          console.error('Failed to save API key:', e);
        }
      }
    }, 500);
  }

  async function handleTest(provider: ProviderConfig) {
    testing = provider.id;
    testResult = null;
    try {
      const key = apiKeys[provider.id] || await getApiKey(provider.name) || '';
      const result = await testConnection(provider.base_url, key, provider.model);
      testResult = {
        success: result.success,
        message: result.success
          ? `Connected! Latency: ${result.latency_ms}ms`
          : result.error ?? 'Connection failed',
      };
    } catch (e) {
      testResult = { success: false, message: `Error: ${e}` };
    } finally {
      testing = null;
    }
  }
</script>

<div class="flex flex-col gap-3 h-full overflow-y-auto p-1">
  {#each appState.providers as provider (provider.id)}
    <GlassCard padding="p-3">
      <div class="flex flex-col gap-2">
        <!-- Header row -->
        <button
          class="flex items-center justify-between w-full text-left cursor-pointer"
          onclick={() => toggleExpand(provider.id)}
        >
          <div class="flex items-center gap-2.5">
            <!-- Active radio indicator -->
            <div
              class="w-4 h-4 rounded-full border-2 flex items-center justify-center shrink-0 transition-colors cursor-pointer
                {appState.activeProviderId === provider.id
                  ? 'border-green-400 bg-green-400/20'
                  : 'border-black/25 dark:border-white/30 hover:border-black/40 dark:hover:border-white/50'}"
              role="radio"
              tabindex="-1"
              aria-checked={appState.activeProviderId === provider.id}
              onclick={(e) => { e.stopPropagation(); setActive(provider.id); }}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); e.preventDefault(); setActive(provider.id); } }}
            >
              {#if appState.activeProviderId === provider.id}
                <div class="w-2 h-2 rounded-full bg-green-400"></div>
              {/if}
            </div>
            <div class="flex flex-col">
              <span class="text-sm font-medium text-black/85 dark:text-white/90">{provider.name}</span>
              <span class="text-[11px] text-black/40 dark:text-white/40 font-mono">{provider.model}</span>
            </div>
          </div>
        </button>

        <!-- Expanded editor -->
        {#if expandedId === provider.id}
          <div class="flex flex-col gap-3 pt-2 border-t border-black/10 dark:border-white/10">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-black/50 dark:text-white/50">Provider Name</span>
              <input
                type="text"
                value={provider.name}
                oninput={(e) => updateProvider(provider.id, 'name', (e.target as HTMLInputElement).value)}
                class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 outline-none focus:border-black/25 dark:focus:border-white/30"
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-black/50 dark:text-white/50">Base URL</span>
              <input
                type="text"
                value={provider.base_url}
                oninput={(e) => updateProvider(provider.id, 'base_url', (e.target as HTMLInputElement).value)}
                placeholder="https://api.openai.com"
                class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-black/50 dark:text-white/50">Model</span>
              <input
                type="text"
                value={provider.model}
                oninput={(e) => updateProvider(provider.id, 'model', (e.target as HTMLInputElement).value)}
                placeholder="gpt-4o-mini"
                class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-black/50 dark:text-white/50">API Key</span>
              <input
                type="password"
                value={apiKeys[provider.id] ?? ''}
                oninput={(e) => handleApiKeyInput(provider.id, (e.target as HTMLInputElement).value)}
                placeholder={apiKeyLoaded[provider.id] ? '••••••••' : 'Loading...'}
                class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-black/50 dark:text-white/50">Timeout: {provider.timeout_secs}s</span>
              <input
                type="range"
                min="5"
                max="120"
                step="5"
                value={provider.timeout_secs}
                oninput={(e) => updateProvider(provider.id, 'timeout_secs', Number((e.target as HTMLInputElement).value))}
                class="w-full accent-blue-400"
              />
            </label>

            <button
              class="w-full py-2 rounded-xl text-sm font-medium bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/15 text-black/60 dark:text-white/70 border border-black/10 dark:border-white/10 disabled:opacity-50 cursor-pointer"
              onclick={() => handleTest(provider)}
              disabled={testing === provider.id}
            >
              {testing === provider.id ? 'Testing...' : 'Test Connection'}
            </button>

            {#if testResult && expandedId === provider.id}
              <div class="text-center text-xs {testResult.success ? 'text-green-600 dark:text-green-300' : 'text-red-600 dark:text-red-300'}">
                {testResult.message}
              </div>
            {/if}

            {#if appState.providers.length > 1}
              <div class="pt-1">
                {#if confirmingDeleteId === provider.id}
                  <div class="flex gap-2">
                    <button
                      class="flex-1 py-1.5 rounded-lg text-xs font-medium bg-red-500/20 hover:bg-red-500/30 text-red-600 dark:text-red-300 border border-red-400/20 cursor-pointer"
                      onclick={() => deleteProvider(provider.id)}
                    >
                      Confirm Delete
                    </button>
                    <button
                      class="flex-1 py-1.5 rounded-lg text-xs font-medium bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/15 text-black/60 dark:text-white/70 border border-black/10 dark:border-white/10 cursor-pointer"
                      onclick={() => confirmingDeleteId = null}
                    >
                      Cancel
                    </button>
                  </div>
                {:else}
                  <button
                    class="w-full py-1.5 rounded-lg text-xs font-medium bg-red-500/10 hover:bg-red-500/20 text-red-500/70 dark:text-red-300/70 border border-red-400/10 cursor-pointer"
                    onclick={() => confirmingDeleteId = provider.id}
                  >
                    Delete Provider
                  </button>
                {/if}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </GlassCard>
  {/each}

  <button
    class="w-full py-2 rounded-xl text-sm font-medium bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/15 text-black/50 dark:text-white/60 border border-dashed border-black/15 dark:border-white/20 cursor-pointer"
    onclick={addProvider}
  >
    + Add Provider
  </button>
</div>
