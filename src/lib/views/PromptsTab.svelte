<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import HotkeyRecorder from '$lib/components/HotkeyRecorder.svelte';
  import { appState } from '$lib/stores/appState.svelte';
  import { registerHotkey, unregisterHotkey } from '$lib/utils/commands';
  import type { RewriteAction } from '$lib/utils/commands';

  let expandedId = $state<string | null>(null);

  function toggleExpand(id: string) {
    expandedId = expandedId === id ? null : id;
  }

  function addAction() {
    const newAction: RewriteAction = {
      id: `action-${Date.now()}`,
      name: 'New Action',
      hotkey: '',
      system_prompt: 'You are a helpful writing assistant. Rewrite the following text.',
      user_template: '{{text}}',
      output_rules: 'Output only the rewritten text. No explanations.',
      enabled: true,
    };
    appState.actions = [...appState.actions, newAction];
    expandedId = newAction.id;
  }

  function deleteAction(id: string) {
    const action = appState.actions.find(a => a.id === id);
    if (action?.hotkey) {
      unregisterHotkey(action.hotkey).catch(() => {});
    }
    appState.actions = appState.actions.filter(a => a.id !== id);
    if (expandedId === id) expandedId = null;
  }

  function moveAction(id: string, direction: -1 | 1) {
    const idx = appState.actions.findIndex(a => a.id === id);
    const newIdx = idx + direction;
    if (newIdx < 0 || newIdx >= appState.actions.length) return;
    const newActions = [...appState.actions];
    [newActions[idx], newActions[newIdx]] = [newActions[newIdx], newActions[idx]];
    appState.actions = newActions;
  }

  async function handleHotkeyChange(action: RewriteAction, newHotkey: string) {
    // Unregister old hotkey
    if (action.hotkey) {
      await unregisterHotkey(action.hotkey).catch(() => {});
    }
    // Update and register new
    const idx = appState.actions.findIndex(a => a.id === action.id);
    if (idx >= 0) {
      const updated = [...appState.actions];
      updated[idx] = { ...updated[idx], hotkey: newHotkey };
      appState.actions = updated;
      if (newHotkey) {
        await registerHotkey(action.id, newHotkey).catch(e => {
          console.error('Failed to register hotkey:', e);
        });
      }
    }
  }

  function updateAction(id: string, field: keyof RewriteAction, value: string | boolean) {
    const idx = appState.actions.findIndex(a => a.id === id);
    if (idx >= 0) {
      const updated = [...appState.actions];
      updated[idx] = { ...updated[idx], [field]: value };
      appState.actions = updated;
    }
  }
</script>

<div class="flex flex-col gap-3 h-full overflow-y-auto p-1">
  <!-- Global enable toggle -->
  <GlassCard padding="p-3">
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
  </GlassCard>

  {#each appState.actions as action (action.id)}
    <GlassCard padding="p-3">
      <div class="flex flex-col gap-2">
        <!-- Header row -->
        <button
          class="flex items-center justify-between w-full text-left cursor-pointer"
          onclick={() => toggleExpand(action.id)}
        >
          <div class="flex items-center gap-2">
            <input
              type="checkbox"
              checked={action.enabled}
              onclick={(e: MouseEvent) => e.stopPropagation()}
              onchange={(e: Event) => updateAction(action.id, 'enabled', (e.target as HTMLInputElement).checked)}
              class="w-4 h-4 rounded bg-white/10 border border-white/20"
            />
            <span class="text-sm font-medium text-white/90">{action.name}</span>
          </div>
          <div class="flex items-center gap-2">
            {#if action.hotkey}
              <span class="px-2 py-0.5 rounded text-xs font-mono bg-white/10 text-white/50">
                {action.hotkey.replace('CommandOrControl', '⌘').replace('Alt', '⌥').replace('Shift', '⇧').replace('Control', '⌃').replace(/\+/g, '')}
              </span>
            {/if}
          </div>
        </button>

        <!-- Expanded editor -->
        {#if expandedId === action.id}
          <div class="flex flex-col gap-3 pt-2 border-t border-white/10">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-white/50">Name</span>
              <input
                type="text"
                value={action.name}
                oninput={(e: Event) => updateAction(action.id, 'name', (e.target as HTMLInputElement).value)}
                class="bg-white/10 border border-white/15 rounded-lg px-3 py-1.5 text-sm text-white/90 outline-none focus:border-white/30"
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-white/50">Hotkey</span>
              <HotkeyRecorder
                value={action.hotkey}
                onchange={(shortcut) => handleHotkeyChange(action, shortcut)}
              />
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-white/50">System Prompt</span>
              <textarea
                value={action.system_prompt}
                oninput={(e: Event) => updateAction(action.id, 'system_prompt', (e.target as HTMLTextAreaElement).value)}
                rows={3}
                class="bg-white/10 border border-white/15 rounded-lg px-3 py-1.5 text-sm text-white/90 outline-none focus:border-white/30 resize-none"
              ></textarea>
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-white/50">User Template <span class="text-white/30">(use {'{{text}}'} for selected text)</span></span>
              <textarea
                value={action.user_template}
                oninput={(e: Event) => updateAction(action.id, 'user_template', (e.target as HTMLTextAreaElement).value)}
                rows={2}
                class="bg-white/10 border border-white/15 rounded-lg px-3 py-1.5 text-sm text-white/90 font-mono outline-none focus:border-white/30 resize-none"
              ></textarea>
            </label>

            <label class="flex flex-col gap-1">
              <span class="text-xs text-white/50">Output Rules</span>
              <input
                type="text"
                value={action.output_rules}
                oninput={(e: Event) => updateAction(action.id, 'output_rules', (e.target as HTMLInputElement).value)}
                class="bg-white/10 border border-white/15 rounded-lg px-3 py-1.5 text-sm text-white/90 outline-none focus:border-white/30"
              />
            </label>

            <div class="flex justify-between pt-1">
              <div class="flex gap-1">
                <button
                  class="px-2 py-1 rounded text-xs bg-white/10 hover:bg-white/15 text-white/50"
                  onclick={() => moveAction(action.id, -1)}
                >↑</button>
                <button
                  class="px-2 py-1 rounded text-xs bg-white/10 hover:bg-white/15 text-white/50"
                  onclick={() => moveAction(action.id, 1)}
                >↓</button>
              </div>
              <button
                class="px-3 py-1 rounded text-xs bg-red-500/20 hover:bg-red-500/30 text-red-300"
                onclick={() => deleteAction(action.id)}
              >Delete</button>
            </div>
          </div>
        {/if}
      </div>
    </GlassCard>
  {/each}

  <button
    class="w-full py-2 rounded-xl text-sm font-medium bg-white/10 hover:bg-white/15 text-white/60 border border-dashed border-white/20"
    onclick={addAction}
  >
    + Add Action
  </button>
</div>
