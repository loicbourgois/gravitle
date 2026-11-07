#[cfg(test)]
#[path = "openrouter_test.rs"]
mod tests;

use serde_json::json;
use std::time::Duration;

pub fn openrouter_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    let path = std::path::PathBuf::from(home)
        .join("github.com/loicbourgois/gravitle_local/openrouter_key");

    read_api_key(&path)
}

fn read_api_key(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path).map_err(|error| {
        format!(
            "Could not read OpenRouter API key file {}: {error}",
            path.display()
        )
    })?;
    let key = contents.trim();

    if key.is_empty() {
        return Err(format!("OpenRouter API key file {} is empty", path.display()).into());
    }

    Ok(key.to_owned())
}

pub fn call_openrouter(
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(2 * 60))
        .build()?;
    // https://openrouter.ai/docs/api_reference/responses/reasoning#reasoning-effort-levels
    let reasoning = match model {
        "openai/gpt-6.1-sol" => json!({
            "effort": "minimal"
        }),
        "anthropic/claude-opus-5.5" => json!({
            "effort": "minimal"
        }),
        "anthropic/claude-sonnet-5.5" => json!({
            "effort": "minimal"
        }),
        "mistralai/mistral-large-4-0" => json!({
            "effort": "minimal"
        }),
        "z-ai/glm-5.3-flashx" => json!({
            "effort": "minimal"
        }),
        "z-ai/glm-5.3-prime" => json!({
            "effort": "minimal"
        }),
        _ => json!({
            "effort": "none"
            , "exclude": true
        }),
    };
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        // .header("X-Title", "Loïc")
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                },
            ],
            "reasoning": reasoning,
            "provider": {
                "sort": "throughput"
                // https://openrouter.ai/docs/guides/routing/provider-selection
                , "allow_fallbacks": true
            },
            "service_tier": "priority",
            "plugins": []
        }))
        .send()?;

    let status = response.status();
    let body = response.text()?;

    if !status.is_success() {
        return Err(format!("OpenRouter request failed with {}: {}", status, body).into());
    }

    parse_response_content(&body)
}

fn parse_response_content(body: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed: serde_json::Value = serde_json::from_str(body)?;

    let content = parsed["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("OpenRouter response is missing message content")?;

    if content.trim().is_empty() {
        return Err("OpenRouter response contains empty message content".into());
    }

    // Validate without changing the model's Markdown formatting.
    Ok(content.to_owned())
}
