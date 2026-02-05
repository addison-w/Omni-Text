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

let provider = $state<ProviderConfig>({
  name: "Default",
  base_url: "https://api.openai.com",
  model: "gpt-4o-mini",
  timeout_secs: 30,
});

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

    const savedProvider = await store.get<ProviderConfig>("provider");
    if (savedProvider) provider = savedProvider;
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
    await store.set("provider", provider);
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

  get provider() { return provider; },
  set provider(v: ProviderConfig) { provider = v; saveState(); },

  loadState,
  saveState,
};
