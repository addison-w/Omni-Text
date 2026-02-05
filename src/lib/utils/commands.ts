import { invoke } from "@tauri-apps/api/core";

// Types matching Rust models
export interface RewriteAction {
  id: string;
  name: string;
  hotkey: string;
  system_prompt: string;
  user_template: string;
  output_rules: string;
  enabled: boolean;
}

export interface ProviderConfig {
  name: string;
  base_url: string;
  model: string;
  timeout_secs: number;
}

export interface HistoryEntry {
  id: string;
  timestamp: string;
  action_name: string;
  app_name: string;
  original_text: string;
  result_text: string;
  provider: string;
  model: string;
  duration_ms: number;
  tokens_used: number | null;
}

export interface LLMResponse {
  text: string;
  tokens_used: number | null;
  duration_ms: number;
}

export interface ConnectionTestResult {
  success: boolean;
  latency_ms: number;
  model_name: string;
  error: string | null;
}

// Text interaction
export async function getSelectedText(): Promise<string> {
  return invoke("get_selected_text");
}

export async function replaceSelectedText(text: string): Promise<void> {
  return invoke("replace_selected_text", { text });
}

export async function checkAccessibilityPermission(): Promise<boolean> {
  return invoke("check_accessibility_permission");
}

export async function requestAccessibilityPermission(): Promise<void> {
  return invoke("request_accessibility_permission");
}

// LLM Provider
export async function callLLM(
  baseUrl: string,
  apiKey: string,
  model: string,
  systemPrompt: string,
  userPrompt: string,
  timeoutSecs: number
): Promise<LLMResponse> {
  return invoke("call_llm", {
    baseUrl,
    apiKey,
    model,
    systemPrompt,
    userPrompt,
    timeoutSecs,
  });
}

export async function testConnection(
  baseUrl: string,
  apiKey: string,
  model: string
): Promise<ConnectionTestResult> {
  return invoke("test_connection", { baseUrl, apiKey, model });
}

// Keychain
export async function saveApiKey(
  providerId: string,
  key: string
): Promise<void> {
  return invoke("save_api_key", { providerId, key });
}

export async function getApiKey(
  providerId: string
): Promise<string | null> {
  return invoke("get_api_key", { providerId });
}

export async function deleteApiKey(providerId: string): Promise<void> {
  return invoke("delete_api_key", { providerId });
}

// Hotkeys
export async function registerHotkey(
  id: string,
  shortcut: string
): Promise<void> {
  return invoke("register_hotkey", { id, shortcut });
}

export async function unregisterHotkey(shortcut: string): Promise<void> {
  return invoke("unregister_hotkey", { shortcut });
}

export async function unregisterAllHotkeys(): Promise<void> {
  return invoke("unregister_all_hotkeys");
}

// History (init handled by SQL plugin migration)
export async function initHistoryDb(): Promise<void> {
  return invoke("init_history_db");
}
