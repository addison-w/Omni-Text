/// Normalize LLM response by stripping common artifacts
///
/// Returns None if the result is empty or identical to the original text.
pub fn normalize(raw: &str, original: &str) -> Option<String> {
    let mut result = raw.to_string();

    // Strip surrounding markdown code blocks
    if result.starts_with("```") {
        // Remove opening ``` line
        if let Some(pos) = result.find('\n') {
            result = result[pos + 1..].to_string();
        }
        // Remove closing ```
        if let Some(pos) = result.rfind("```") {
            result = result[..pos].to_string();
        }
    }

    // Strip surrounding quotes (single or double)
    let trimmed = result.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        result = trimmed[1..trimmed.len() - 1].to_string();
    }

    // Strip common LLM prefixes
    let prefixes = [
        "Here's your rewrite:",
        "Here's the rewritten text:",
        "Here is your rewrite:",
        "Here is the rewritten text:",
        "Rewritten text:",
        "Sure, here's the rewrite:",
        "Sure! Here's the rewrite:",
        "Here you go:",
    ];
    for prefix in &prefixes {
        if let Some(stripped) = result
            .trim()
            .strip_prefix(prefix)
        {
            result = stripped.to_string();
            break;
        }
    }

    // Final trim
    let result = result.trim().to_string();

    // Return None if empty or identical to input
    if result.is_empty() || result == original.trim() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strips_markdown_code_blocks() {
        let raw = "```\nHello world\n```";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }

    #[test]
    fn test_strips_language_tagged_code_blocks() {
        let raw = "```text\nHello world\n```";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }

    #[test]
    fn test_strips_surrounding_double_quotes() {
        let raw = "\"Hello world\"";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }

    #[test]
    fn test_strips_surrounding_single_quotes() {
        let raw = "'Hello world'";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }

    #[test]
    fn test_strips_llm_prefix() {
        let raw = "Here's your rewrite: Hello world";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }

    #[test]
    fn test_returns_none_for_empty() {
        assert_eq!(normalize("", "original"), None);
        assert_eq!(normalize("   ", "original"), None);
    }

    #[test]
    fn test_returns_none_for_identical() {
        assert_eq!(normalize("hello", "hello"), None);
        assert_eq!(normalize("  hello  ", "hello"), None);
    }

    #[test]
    fn test_passthrough_normal_text() {
        let raw = "This is a perfectly normal rewrite.";
        assert_eq!(
            normalize(raw, "original text"),
            Some("This is a perfectly normal rewrite.".into())
        );
    }

    #[test]
    fn test_combined_artifacts() {
        let raw = "```\n\"Here's your rewrite: Hello world\"\n```";
        assert_eq!(normalize(raw, "original"), Some("Hello world".into()));
    }
}
