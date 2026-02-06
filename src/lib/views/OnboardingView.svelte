<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import {
    saveApiKey,
    testConnection,
    checkAccessibilityPermission,
    requestAccessibilityPermission,
  } from '$lib/utils/commands';

  interface Props {
    onComplete: () => void;
  }

  let { onComplete }: Props = $props();

  let step = $state(0);
  let baseUrl = $state(appState.provider.base_url);
  let model = $state(appState.provider.model);
  let apiKey = $state('');
  let testing = $state(false);
  let testResult = $state<{ success: boolean; message: string } | null>(null);
  let accessibilityGranted = $state(false);

  const steps = ['Welcome', 'Provider', 'Accessibility', 'Ready'];

  function saveProviderSettings() {
    const idx = appState.providers.findIndex(p => p.id === appState.activeProviderId);
    if (idx >= 0) {
      const updated = [...appState.providers];
      updated[idx] = { ...updated[idx], base_url: baseUrl, model: model };
      appState.providers = updated;
    }
  }

  async function handleTestAndNext() {
    testing = true;
    testResult = null;
    try {
      saveProviderSettings();
      if (apiKey) {
        await saveApiKey(appState.provider.name, apiKey);
      }
      const result = await testConnection(baseUrl, apiKey, model);
      if (result.success) {
        testResult = { success: true, message: `Connected! (${result.latency_ms}ms)` };
        setTimeout(() => { step = 2; }, 800);
      } else {
        testResult = { success: false, message: result.error ?? 'Connection failed' };
      }
    } catch (e) {
      testResult = { success: false, message: `Error: ${e}` };
    } finally {
      testing = false;
    }
  }

  function handleSkip() {
    saveProviderSettings();
    step = 2;
  }

  async function checkAccessibility() {
    accessibilityGranted = await checkAccessibilityPermission();
  }

  async function handleRequestAccess() {
    await requestAccessibilityPermission();
    // Poll a few times since the user needs to go to System Settings
    for (let i = 0; i < 10; i++) {
      await new Promise(r => setTimeout(r, 2000));
      accessibilityGranted = await checkAccessibilityPermission();
      if (accessibilityGranted) break;
    }
  }

  $effect(() => {
    if (step === 2) {
      checkAccessibility();
    }
  });
</script>

<div class="h-full flex flex-col items-center justify-center p-6">
  <!-- Step indicator -->
  <div class="flex gap-2 mb-6">
    {#each steps as s, i}
      <div class="w-2 h-2 rounded-full transition-all {i <= step ? 'bg-black/50 dark:bg-white/60' : 'bg-black/10 dark:bg-white/15'}"></div>
    {/each}
  </div>

  {#if step === 0}
    <!-- Welcome -->
    <div class="text-center flex flex-col gap-4 max-w-sm">
      <h1 class="text-xl font-semibold">Welcome to Omni Text</h1>
      <p class="text-sm text-black/50 dark:text-white/60">
        Select any text, press a hotkey, and get it rewritten by AI — right in place.
      </p>
      <button
        class="mt-4 py-2.5 rounded-xl text-sm font-medium bg-black/8 dark:bg-white/15 hover:bg-black/12 dark:hover:bg-white/20 text-black/80 dark:text-white/90 border border-black/10 dark:border-white/10"
        onclick={() => step = 1}
      >
        Get Started
      </button>
    </div>

  {:else if step === 1}
    <!-- Provider setup -->
    <div class="w-full max-w-sm flex flex-col gap-3">
      <h2 class="text-lg font-semibold text-center">Set Up Your Provider</h2>
      <p class="text-xs text-black/45 dark:text-white/50 text-center">Any OpenAI-compatible API works (OpenAI, Ollama, LM Studio, etc.)</p>

      <GlassCard padding="p-4">
        <div class="flex flex-col gap-3">
          <label class="flex flex-col gap-1">
            <span class="text-xs text-black/50 dark:text-white/50">Base URL</span>
            <input
              type="text"
              bind:value={baseUrl}
              placeholder="https://api.openai.com"
              class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
            />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-black/50 dark:text-white/50">Model</span>
            <input
              type="text"
              bind:value={model}
              placeholder="gpt-4o-mini"
              class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
            />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-black/50 dark:text-white/50">API Key</span>
            <input
              type="password"
              bind:value={apiKey}
              placeholder="sk-..."
              class="bg-black/5 dark:bg-white/10 border border-black/10 dark:border-white/15 rounded-lg px-3 py-2 text-sm text-black/85 dark:text-white/90 font-mono outline-none focus:border-black/25 dark:focus:border-white/30"
            />
          </label>
        </div>
      </GlassCard>

      <button
        class="py-2.5 rounded-xl text-sm font-medium bg-blue-500/20 hover:bg-blue-500/30 text-blue-700 dark:text-blue-200 border border-blue-400/20 disabled:opacity-50"
        onclick={handleTestAndNext}
        disabled={testing}
      >
        {testing ? 'Testing...' : 'Test & Continue'}
      </button>

      {#if testResult}
        <p class="text-xs text-center {testResult.success ? 'text-green-600 dark:text-green-300' : 'text-red-600 dark:text-red-300'}">
          {testResult.message}
        </p>
      {/if}

      <button
        class="text-xs text-black/30 dark:text-white/30 hover:text-black/50 dark:hover:text-white/50"
        onclick={handleSkip}
      >
        Skip for now
      </button>
    </div>

  {:else if step === 2}
    <!-- Accessibility -->
    <div class="text-center flex flex-col gap-4 max-w-sm">
      <h2 class="text-lg font-semibold">Accessibility Permission</h2>
      <p class="text-sm text-black/50 dark:text-white/60">
        Omni Text needs accessibility access to read and replace your selected text.
      </p>

      <div class="flex items-center justify-center gap-2 py-2">
        <div class="w-3 h-3 rounded-full {accessibilityGranted ? 'bg-green-400' : 'bg-red-400'}"></div>
        <span class="text-sm {accessibilityGranted ? 'text-green-600 dark:text-green-300' : 'text-black/50 dark:text-white/60'}">
          {accessibilityGranted ? 'Permission Granted' : 'Not yet granted'}
        </span>
      </div>

      {#if !accessibilityGranted}
        <button
          class="py-2.5 rounded-xl text-sm font-medium bg-blue-500/20 hover:bg-blue-500/30 text-blue-700 dark:text-blue-200 border border-blue-400/20"
          onclick={handleRequestAccess}
        >
          Open Accessibility Settings
        </button>
        <p class="text-xs text-black/30 dark:text-white/30">
          System Settings → Privacy & Security → Accessibility → Enable Omni Text
        </p>
      {/if}

      <button
        class="py-2.5 rounded-xl text-sm font-medium bg-black/8 dark:bg-white/15 hover:bg-black/12 dark:hover:bg-white/20 text-black/80 dark:text-white/90 border border-black/10 dark:border-white/10"
        onclick={() => step = 3}
      >
        {accessibilityGranted ? 'Continue' : 'Skip for now'}
      </button>
    </div>

  {:else if step === 3}
    <!-- Ready -->
    <div class="text-center flex flex-col gap-4 max-w-sm">
      <h2 class="text-xl font-semibold">You're All Set!</h2>
      <p class="text-sm text-black/50 dark:text-white/60">
        Select any text and press <span class="font-mono bg-black/5 dark:bg-white/10 px-1.5 py-0.5 rounded">&#x2318;&#x21E7;1</span> to proofread or <span class="font-mono bg-black/5 dark:bg-white/10 px-1.5 py-0.5 rounded">&#x2318;&#x21E7;2</span> to rewrite it.
      </p>
      <p class="text-xs text-black/40 dark:text-white/40">
        You can customize actions and hotkeys in Settings.
      </p>
      <button
        class="mt-2 py-2.5 rounded-xl text-sm font-medium bg-green-500/20 hover:bg-green-500/30 text-green-700 dark:text-green-200 border border-green-400/20"
        onclick={onComplete}
      >
        Start Using Omni Text
      </button>
    </div>
  {/if}
</div>
