<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import MenuBarPopover from '$lib/components/MenuBarPopover.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import PromptsTab from '$lib/views/PromptsTab.svelte';
  import ProviderTab from '$lib/views/ProviderTab.svelte';
  import HistoryTab from '$lib/views/HistoryTab.svelte';
  import PrivacyTab from '$lib/views/PrivacyTab.svelte';
  import AdvancedTab from '$lib/views/AdvancedTab.svelte';
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

  let currentView = $state<'popover' | 'settings' | 'onboarding'>('popover');
  let settingsTab = $state<'prompts' | 'provider' | 'history' | 'privacy' | 'advanced'>('prompts');

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

  async function resizeWindow(width: number, height: number) {
    try {
      const win = getCurrentWindow();
      await win.setSize(new (await import('@tauri-apps/api/dpi')).LogicalSize(width, height));
    } catch (e) {
      console.error('Failed to resize:', e);
    }
  }

  function openSettings() {
    currentView = 'settings';
    resizeWindow(600, 700);
  }

  function closeSettings() {
    currentView = 'popover';
    resizeWindow(360, 520);
  }

  // Phase 8: Full rewrite pipeline
  async function handleHotkeyTriggered(actionId: string) {
    if (!appState.isEnabled || appState.isProcessing) return;

    const action = appState.actions.find(a => a.id === actionId && a.enabled);
    if (!action) return;

    appState.isProcessing = true;
    appState.status = 'processing';

    try {
      // Step 1: Get selected text
      const selectedText = await getSelectedText();
      if (!selectedText || selectedText.trim().length === 0) {
        showToast('Select text first', 'error');
        return;
      }

      // Step 2: Build prompt
      const userPrompt = action.user_template.replace('{{text}}', selectedText);
      const systemPrompt = action.system_prompt +
        (action.output_rules ? '\n\n' + action.output_rules : '');

      // Step 3: Get API key
      const apiKey = await getApiKey(appState.provider.name);
      if (!apiKey) {
        showToast('No API key configured. Open Settings.', 'error');
        return;
      }

      // Step 4: Call LLM
      const response = await callLLM(
        appState.provider.base_url,
        apiKey,
        appState.provider.model,
        systemPrompt,
        userPrompt,
        appState.provider.timeout_secs
      );

      // Step 5: Replace selected text
      await replaceSelectedText(response.text);

      // Step 6: Save to history (if not privacy mode)
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

  onMount(async () => {
    // Initialize stores
    await appState.loadState();
    await themeStore.initTheme();

    // Check onboarding
    if (!appState.hasCompletedOnboarding) {
      currentView = 'onboarding';
      return;
    }

    // Register hotkeys for all enabled actions
    for (const action of appState.actions) {
      if (action.enabled && action.hotkey) {
        try {
          await registerHotkey(action.id, action.hotkey);
        } catch (e) {
          console.error(`Failed to register hotkey for ${action.name}:`, e);
        }
      }
    }

    // Listen for hotkey events from Rust
    await listen<string>('hotkey-triggered', (event) => {
      handleHotkeyTriggered(event.payload);
    });
  });

  async function handleOnboardingComplete() {
    appState.hasCompletedOnboarding = true;
    currentView = 'popover';
    resizeWindow(360, 520);
    // Register hotkeys after onboarding
    for (const action of appState.actions) {
      if (action.enabled && action.hotkey) {
        try {
          await registerHotkey(action.id, action.hotkey);
        } catch (e) {
          console.error(`Failed to register hotkey for ${action.name}:`, e);
        }
      }
    }
    // Listen for hotkey events from Rust
    await listen<string>('hotkey-triggered', (event) => {
      handleHotkeyTriggered(event.payload);
    });
  }

  const tabs = [
    { id: 'prompts', label: 'Actions' },
    { id: 'provider', label: 'Provider' },
    { id: 'history', label: 'History' },
    { id: 'privacy', label: 'Privacy' },
    { id: 'advanced', label: 'Advanced' },
  ] as const;
</script>

<div class="h-full w-full flex flex-col text-white/90 {themeStore.theme === 'dark' ? 'dark' : ''}">
  <!-- Toast overlay -->
  <Toast message={toastMessage} variant={toastVariant} visible={toastVisible} />

  {#if currentView === 'onboarding'}
    <OnboardingView onComplete={handleOnboardingComplete} />
  {:else if currentView === 'popover'}
    <MenuBarPopover onOpenSettings={openSettings} />
  {:else if currentView === 'settings'}
    <!-- Settings view -->
    <div class="flex flex-col h-full">
      <!-- Settings header -->
      <div class="flex items-center justify-between p-4 pb-2">
        <button
          class="text-sm text-white/60 hover:text-white/90 flex items-center gap-1"
          onclick={closeSettings}
        >
          ← Back
        </button>
        <span class="text-sm font-semibold text-white/90">Settings</span>
        <div class="w-10"></div>
      </div>

      <!-- Tab bar -->
      <div class="flex px-4 gap-1 border-b border-white/10 pb-0">
        {#each tabs as tab}
          <button
            class="px-3 py-2 text-xs font-medium transition-all border-b-2
              {settingsTab === tab.id
                ? 'border-white/60 text-white/90'
                : 'border-transparent text-white/40 hover:text-white/60'}"
            onclick={() => settingsTab = tab.id}
          >
            {tab.label}
          </button>
        {/each}
      </div>

      <!-- Tab content -->
      <div class="flex-1 overflow-y-auto p-3">
        {#if settingsTab === 'prompts'}
          <PromptsTab />
        {:else if settingsTab === 'provider'}
          <ProviderTab />
        {:else if settingsTab === 'history'}
          <HistoryTab />
        {:else if settingsTab === 'privacy'}
          <PrivacyTab />
        {:else if settingsTab === 'advanced'}
          <AdvancedTab />
        {/if}
      </div>
    </div>
  {/if}
</div>
