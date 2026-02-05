use crate::models::*;
use crate::services::response_normalizer;
use std::time::Instant;

#[tauri::command]
pub async fn call_llm(
    base_url: String,
    api_key: String,
    model: String,
    system_prompt: String,
    user_prompt: String,
    timeout_secs: u64,
) -> Result<LLMResponse, String> {
    let start = Instant::now();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

    let request_body = ChatCompletionRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".into(),
                content: user_prompt.clone(),
            },
        ],
        temperature: 0.3,
        max_tokens: Some(4096),
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                "Request timed out. Check your provider URL and network connection.".to_string()
            } else if e.is_connect() {
                format!("Connection failed: {}. Check your provider URL.", e)
            } else {
                format!("Request failed: {}", e)
            }
        })?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(match status.as_u16() {
            401 => "Invalid API key. Check your API key in settings.".into(),
            403 => "Access denied. Your API key may not have permission for this model.".into(),
            404 => format!("Model not found. Check the model name in settings. Response: {}", body),
            429 => "Rate limited. Please wait and try again.".into(),
            500..=599 => format!("Provider server error ({}): {}", status, body),
            _ => format!("API error ({}): {}", status, body),
        });
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    let completion: ChatCompletionResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let raw_text = completion
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    let tokens_used = completion
        .usage
        .and_then(|u| u.total_tokens);

    // Extract original text from user prompt for normalization
    // The user_prompt contains the template with the original text
    let normalized = response_normalizer::normalize(&raw_text, &user_prompt)
        .unwrap_or(raw_text);

    Ok(LLMResponse {
        text: normalized,
        tokens_used,
        duration_ms,
    })
}

#[tauri::command]
pub async fn test_connection(
    base_url: String,
    api_key: String,
    model: String,
) -> Result<ConnectionTestResult, String> {
    let start = Instant::now();

    let result = call_llm(
        base_url,
        api_key,
        model.clone(),
        "You are a test assistant.".into(),
        "Reply with exactly: OK".into(),
        10,
    )
    .await;

    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => Ok(ConnectionTestResult {
            success: true,
            latency_ms,
            model_name: model,
            error: None,
        }),
        Err(e) => Ok(ConnectionTestResult {
            success: false,
            latency_ms,
            model_name: model,
            error: Some(e),
        }),
    }
}
