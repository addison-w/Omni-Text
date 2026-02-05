use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum AppStatus {
    Ready,
    Processing,
    NotReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RewriteAction {
    pub id: String,
    pub name: String,
    pub hotkey: String,
    pub system_prompt: String,
    pub user_template: String,
    pub output_rules: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub model: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub action_name: String,
    pub app_name: String,
    pub original_text: String,
    pub result_text: String,
    pub provider: String,
    pub model: String,
    pub duration_ms: u64,
    pub tokens_used: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
    pub tokens_used: Option<u64>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub latency_ms: u64,
    pub model_name: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TextInteractionError {
    AccessibilityDenied,
    NoSelection,
    SecureField,
    ClipboardError(String),
    Unknown(String),
}

impl std::fmt::Display for TextInteractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessibilityDenied => write!(f, "Accessibility permission not granted"),
            Self::NoSelection => write!(f, "No text selected"),
            Self::SecureField => write!(f, "Cannot interact with secure/password fields"),
            Self::ClipboardError(msg) => write!(f, "Clipboard error: {}", msg),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

// OpenAI-compatible API types
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Debug, Deserialize)]
pub struct ChatUsage {
    pub total_tokens: Option<u64>,
}
