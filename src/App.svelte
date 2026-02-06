<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import Toast from '$lib/components/Toast.svelte';
  import StatusIndicator from '$lib/components/StatusIndicator.svelte';
  import PromptsTab from '$lib/views/PromptsTab.svelte';
  import ProviderTab from '$lib/views/ProviderTab.svelte';
  import HistoryTab from '$lib/views/HistoryTab.svelte';
  import SettingsTab from '$lib/views/SettingsTab.svelte';
  import OnboardingView from '$lib/views/OnboardingView.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { themeStore } from '$lib/stores/theme.svelte';
  import {
    getSelectedText,
    replaceSelectedText,
    callLLM,
    getApiKey,
    registerHotkey,
  } from '$lib/utils/commands';
  import Database from '@tauri-apps/plugin-sql';

  let showOnboarding = $state(false);
  let activeTab = $state<'actions' | 'provider' | 'history' | 'settings'>('actions');

  // Toast state
  let toastMessage = $state('');
  let toastVariant = $state<'success' | 'error' | 'info'>('info');
  let toastVisible = $state(false);
  let toastTimeout: ReturnType<typeof setTimeout>;

  function showToast(message: string, variant: 'success' | 'error' | 'info' = 'info') {
    toastMessage = message;
    toastVariant = variant;
    toastVisible = true;
    clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => { toastVisible = false; }, 3000);
  }

  async function handleHotkeyTriggered(actionId: string) {
    if (!appState.isEnabled || appState.isProcessing) return;

    const action = appState.actions.find(a => a.id === actionId && a.enabled);
    if (!action) return;

    appState.isProcessing = true;
    appState.status = 'processing';

    try {
      const selectedText = await getSelectedText();
      if (!selectedText || selectedText.trim().length === 0) {
        showToast('Select text first', 'error');
        return;
      }

      const userPrompt = action.user_template.replace('{{text}}', selectedText);
      const systemPrompt = action.system_prompt +
        (action.output_rules ? '\n\n' + action.output_rules : '');

      const apiKey = await getApiKey(appState.provider.name);
      if (!apiKey) {
        showToast('No API key configured. Go to Provider tab.', 'error');
        return;
      }

      const response = await callLLM(
        appState.provider.base_url,
        apiKey,
        appState.provider.model,
        systemPrompt,
        userPrompt,
        appState.provider.timeout_secs
      );

      await replaceSelectedText(response.text);

      if (!appState.privacyMode) {
        try {
          const db = await Database.load('sqlite:omni_text_history.db');
          await db.execute(
            `INSERT INTO history (id, timestamp, action_name, app_name, original_text, result_text, provider, model, duration_ms, tokens_used)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)`,
            [
              crypto.randomUUID(),
              new Date().toISOString(),
              action.name,
              '',
              selectedText,
              response.text,
              appState.provider.name,
              appState.provider.model,
              response.duration_ms,
              response.tokens_used,
            ]
          );
        } catch (e) {
          console.error('Failed to save history:', e);
        }
      }

      showToast(`Rewritten (${response.duration_ms}ms). ⌘Z to undo.`, 'success');
    } catch (e) {
      const error = String(e);
      showToast(error, 'error');
      appState.currentError = error;
    } finally {
      appState.isProcessing = false;
      appState.status = 'ready';
    }
  }

  async function registerHotkeysAndListen() {
    for (const action of appState.actions) {
      if (action.enabled && action.hotkey) {
        try {
          await registerHotkey(action.id, action.hotkey);
        } catch (e) {
          console.error(`Failed to register hotkey for ${action.name}:`, e);
        }
      }
    }
    await listen<string>('hotkey-triggered', (event) => {
      handleHotkeyTriggered(event.payload);
    });
  }

  onMount(async () => {
    await appState.loadState();
    await themeStore.initTheme();

    if (!appState.hasCompletedOnboarding) {
      showOnboarding = true;
      return;
    }

    await registerHotkeysAndListen();
  });

  async function handleOnboardingComplete() {
    appState.hasCompletedOnboarding = true;
    showOnboarding = false;
    await registerHotkeysAndListen();
  }

  const tabs = [
    { id: 'actions', label: 'Actions' },
    { id: 'provider', label: 'Provider' },
    { id: 'history', label: 'History' },
    { id: 'settings', label: 'Settings' },
  ] as const;
</script>

<div class="h-full w-full flex flex-col text-white/90 {themeStore.theme === 'dark' ? 'dark' : ''}">
  <Toast message={toastMessage} variant={toastVariant} visible={toastVisible} />

  {#if showOnboarding}
    <OnboardingView onComplete={handleOnboardingComplete} />
  {:else}
    <div class="flex flex-col h-full">
      <!-- Header -->
      <div class="flex items-center justify-between px-4 pt-3 pb-1">
        <span class="text-sm font-semibold text-white/90">Omni Text</span>
        <StatusIndicator status={appState.status} />
      </div>

      <!-- Tab bar -->
      <div class="flex px-4 gap-1 border-b border-white/10">
        {#each tabs as tab}
          <button
            class="px-3 py-2 text-xs font-medium transition-colors border-b-2 cursor-pointer
              {activeTab === tab.id
                ? 'border-white/60 text-white/90'
                : 'border-transparent text-white/40 hover:text-white/60'}"
            onclick={() => activeTab = tab.id}
          >
            {tab.label}
          </button>
        {/each}
      </div>

      <!-- Tab content -->
      <div class="flex-1 overflow-y-auto p-3">
        {#if activeTab === 'actions'}
          <PromptsTab />
        {:else if activeTab === 'provider'}
          <ProviderTab />
        {:else if activeTab === 'history'}
          <HistoryTab />
        {:else if activeTab === 'settings'}
          <SettingsTab />
        {/if}
      </div>
    </div>
  {/if}
</div>
