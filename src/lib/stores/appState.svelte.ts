import { load } from "@tauri-apps/plugin-store";
import type { RewriteAction, ProviderConfig } from "$lib/utils/commands";

const STORE_PATH = "settings.json";

// App state using Svelte 5 runes
let isEnabled = $state(true);
let isProcessing = $state(false);
let status = $state<"ready" | "processing" | "not-ready">("ready");
let currentError = $state<string | null>(null);
let privacyMode = $state(false);
let hasCompletedOnboarding = $state(false);

let actions = $state<RewriteAction[]>([
  {
    id: "default-proofread",
    name: "Proofread",
    hotkey: "CommandOrControl+Shift+1",
    system_prompt:
      "You are a meticulous proofreader. Fix all spelling, grammar, and punctuation errors. Preserve the original tone and meaning. Only output the corrected text, nothing else.",
    user_template: "{{text}}",
    output_rules: "Output only the corrected text. No explanations.",
    enabled: true,
  },
  {
    id: "default-rewrite",
    name: "Rewrite",
    hotkey: "CommandOrControl+Shift+2",
    system_prompt:
      "You are a skilled editor. Rewrite the given text to improve clarity, readability, and flow. Simplify complex sentences, remove ambiguity, and make the meaning immediately clear. Preserve the original intent and information. Only output the rewritten text, nothing else.",
    user_template: "{{text}}",
    output_rules: "Output only the rewritten text. No explanations.",
    enabled: true,
  },
]);

let providers = $state<ProviderConfig[]>([
  {
    id: "default",
    name: "OpenAI",
    base_url: "https://api.openai.com",
    model: "gpt-4o-mini",
    timeout_secs: 30,
  },
]);

let activeProviderId = $state("default");

// Store instance (lazy loaded)
let storeInstance: Awaited<ReturnType<typeof load>> | null = null;

async function getStore() {
  if (!storeInstance) {
    storeInstance = await load(STORE_PATH, { autoSave: true });
  }
  return storeInstance;
}

async function loadState() {
  try {
    const store = await getStore();

    const savedEnabled = await store.get<boolean>("isEnabled");
    if (savedEnabled !== null && savedEnabled !== undefined) isEnabled = savedEnabled;

    const savedPrivacy = await store.get<boolean>("privacyMode");
    if (savedPrivacy !== null && savedPrivacy !== undefined) privacyMode = savedPrivacy;

    const savedOnboarding = await store.get<boolean>("hasCompletedOnboarding");
    if (savedOnboarding !== null && savedOnboarding !== undefined)
      hasCompletedOnboarding = savedOnboarding;

    const savedActions = await store.get<RewriteAction[]>("actions");
    if (savedActions && savedActions.length > 0) actions = savedActions;

    // Load multi-provider state, with migration from single provider
    const savedProviders = await store.get<ProviderConfig[]>("providers");
    const savedActiveId = await store.get<string>("activeProviderId");

    if (savedProviders && savedProviders.length > 0) {
      providers = savedProviders;
      if (savedActiveId) activeProviderId = savedActiveId;
      else activeProviderId = savedProviders[0].id;
    } else {
      // Migrate from old single-provider format
      const savedProvider = await store.get<{ name: string; base_url: string; model: string; timeout_secs: number }>("provider");
      if (savedProvider) {
        providers = [{ id: "default", ...savedProvider }];
        activeProviderId = "default";
      }
    }
  } catch (e) {
    console.error("Failed to load state:", e);
  }
}

async function saveState() {
  try {
    const store = await getStore();
    await store.set("isEnabled", isEnabled);
    await store.set("privacyMode", privacyMode);
    await store.set("hasCompletedOnboarding", hasCompletedOnboarding);
    await store.set("actions", actions);
    await store.set("providers", providers);
    await store.set("activeProviderId", activeProviderId);
  } catch (e) {
    console.error("Failed to save state:", e);
  }
}

export const appState = {
  get isEnabled() { return isEnabled; },
  set isEnabled(v: boolean) { isEnabled = v; saveState(); },

  get isProcessing() { return isProcessing; },
  set isProcessing(v: boolean) { isProcessing = v; },

  get status() { return status; },
  set status(v: "ready" | "processing" | "not-ready") { status = v; },

  get currentError() { return currentError; },
  set currentError(v: string | null) { currentError = v; },

  get privacyMode() { return privacyMode; },
  set privacyMode(v: boolean) { privacyMode = v; saveState(); },

  get hasCompletedOnboarding() { return hasCompletedOnboarding; },
  set hasCompletedOnboarding(v: boolean) { hasCompletedOnboarding = v; saveState(); },

  get actions() { return actions; },
  set actions(v: RewriteAction[]) { actions = v; saveState(); },

  get providers() { return providers; },
  set providers(v: ProviderConfig[]) { providers = v; saveState(); },

  get activeProviderId() { return activeProviderId; },
  set activeProviderId(v: string) { activeProviderId = v; saveState(); },

  /** Active provider (computed from providers + activeProviderId) */
  get provider(): ProviderConfig {
    return providers.find(p => p.id === activeProviderId) ?? providers[0];
  },

  loadState,
  saveState,
};
