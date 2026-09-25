//! Conversational Co-Pilot gateway.
//!
//! Implements the server side of `epic_02_conversational_copilot_panel`: the Multi-LLM Gateway
//! whose first concrete provider is the Google Gemini REST API. The Rust binary is the only
//! party that ever holds a provider credential (R3) and the only party that resolves the bound
//! document from disk (R5); the SPA merely consumes a normalized Server-Sent Events stream.

use serde::{Deserialize, Serialize};

/// Provider slug of the first (and currently only) implemented gateway backend.
pub const PROVIDER_GEMINI: &str = "gemini";

/// Default Gemini model used when the workspace has not selected one explicitly.
pub const DEFAULT_MODEL: &str = "gemini-2.5-flash";

/// Default Gemini REST base URL. Overridable via `LIQUID_COPILOT_BASE_URL` (used by tests to
/// point the gateway at a local stub provider).
pub const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com";

/// SQLite `settings` keys backing the provider credential and model selection.
pub const SETTING_API_KEY: &str = "copilot.api_key";
pub const SETTING_MODEL: &str = "copilot.model";

/// Environment fallback for the provider credential, honoured when the database has none.
pub const ENV_API_KEY: &str = "GEMINI_API_KEY";
pub const ENV_BASE_URL: &str = "LIQUID_COPILOT_BASE_URL";

/// Upper bound of bound-document characters forwarded as context on each turn. Keeps a single
/// turn bounded regardless of how large the canvas document is.
pub const MAX_CONTEXT_CHARS: usize = 24_000;

/// Where the credential in force came from, reported to the client without disclosing it (R3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CredentialSource {
    Database,
    Environment,
    None,
}

/// A single conversation turn as sent by the co-pilot panel.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub text: String,
}

/// Payload accepted by `POST /api/copilot/chat`.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    /// Workspace-relative path of the document bound to the canvas (R1). Resolved server-side
    /// through the workspace path guard (R5).
    #[serde(default)]
    pub context_path: Option<String>,
}

/// Runtime configuration of the gateway. Held in `AppState` so tests can inject a stub provider.
#[derive(Debug, Clone)]
pub struct CopilotConfig {
    pub base_url: String,
    pub default_model: String,
}

impl CopilotConfig {
    /// Builds the configuration from the environment, falling back to the public Gemini endpoint.
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var(ENV_BASE_URL)
                .ok()
                .map(|v| v.trim_end_matches('/').to_string())
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            default_model: DEFAULT_MODEL.to_string(),
        }
    }

    /// Absolute URL of the Gemini streaming endpoint for a given model.
    pub fn stream_endpoint(&self, model: &str) -> String {
        format!(
            "{}/v1beta/models/{}:streamGenerateContent?alt=sse",
            self.base_url.trim_end_matches('/'),
            model
        )
    }
}

impl Default for CopilotConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Validation failures of an inbound chat request, rejected at the boundary (fail-fast, R4).
#[derive(Debug, PartialEq, Eq)]
pub enum ChatRequestError {
    EmptyConversation,
    LastMessageNotFromUser,
    EmptyPrompt,
}

impl ChatRequestError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyConversation => "EMPTY_CONVERSATION",
            Self::LastMessageNotFromUser => "INVALID_TURN_ORDER",
            Self::EmptyPrompt => "EMPTY_PROMPT",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::EmptyConversation => "The conversation must contain at least one message",
            Self::LastMessageNotFromUser => "The last message of a turn must have role 'user'",
            Self::EmptyPrompt => "The user prompt must not be empty",
        }
    }
}

/// Rejects malformed conversations before any outbound call is attempted.
pub fn validate_chat_request(request: &ChatRequest) -> Result<(), ChatRequestError> {
    let last = request
        .messages
        .last()
        .ok_or(ChatRequestError::EmptyConversation)?;

    if normalize_role(&last.role) != "user" {
        return Err(ChatRequestError::LastMessageNotFromUser);
    }

    if last.text.trim().is_empty() {
        return Err(ChatRequestError::EmptyPrompt);
    }

    Ok(())
}

/// Maps a panel role onto a Gemini content role. Gemini only knows `user` and `model`.
fn normalize_role(role: &str) -> &'static str {
    match role.trim().to_ascii_lowercase().as_str() {
        "model" | "assistant" | "copilot" => "model",
        _ => "user",
    }
}

/// Builds the system instruction that binds the active canvas document to the turn (R1).
pub fn build_system_instruction(
    context_path: Option<&str>,
    context_content: Option<&str>,
) -> String {
    let mut instruction = String::from(
        "You are the Liquid ADE Conversational Co-Pilot, assisting a product architect inside an \
         SCPE (Spec-Compiled Product Engineering) workspace where living Markdown specifications \
         in Git are the Single Source of Truth. Answer concisely in Markdown. When proposing \
         changes to a specification, quote the exact section you would rewrite.",
    );

    match (context_path, context_content) {
        (Some(path), Some(content)) => {
            let excerpt = truncate_context(content);
            instruction.push_str(&format!(
                "\n\nThe document currently bound to the canvas is `{path}`. Its content follows \
                 between the markers.\n\n--- BEGIN {path} ---\n{excerpt}\n--- END {path} ---"
            ));
        }
        (Some(path), None) => {
            instruction.push_str(&format!(
                "\n\nThe document currently bound to the canvas is `{path}`, but its content could \
                 not be read."
            ));
        }
        _ => {
            instruction
                .push_str("\n\nNo document is currently bound to the canvas; answer generally.");
        }
    }

    instruction
}

/// Clamps document context to [`MAX_CONTEXT_CHARS`] on a character boundary.
fn truncate_context(content: &str) -> String {
    if content.chars().count() <= MAX_CONTEXT_CHARS {
        return content.to_string();
    }
    let mut excerpt: String = content.chars().take(MAX_CONTEXT_CHARS).collect();
    excerpt.push_str("\n\n[... document truncated by Liquid ADE ...]");
    excerpt
}

/// Builds the Gemini `streamGenerateContent` request body for one conversation turn.
pub fn build_gemini_payload(
    request: &ChatRequest,
    context_content: Option<&str>,
) -> serde_json::Value {
    let contents: Vec<serde_json::Value> = request
        .messages
        .iter()
        .filter(|m| !m.text.trim().is_empty())
        .map(|m| {
            serde_json::json!({
                "role": normalize_role(&m.role),
                "parts": [{ "text": m.text }],
            })
        })
        .collect();

    serde_json::json!({
        "systemInstruction": {
            "parts": [{
                "text": build_system_instruction(request.context_path.as_deref(), context_content),
            }],
        },
        "contents": contents,
    })
}

/// Extracts the incremental text carried by one Gemini SSE `data:` payload.
///
/// Returns `None` for keep-alive or metadata-only chunks so the gateway never emits empty tokens.
pub fn extract_text_from_gemini_chunk(chunk: &serde_json::Value) -> Option<String> {
    let parts = chunk
        .get("candidates")?
        .as_array()?
        .first()?
        .get("content")?
        .get("parts")?
        .as_array()?;

    let text: String = parts
        .iter()
        .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
        .collect();

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Incremental parser for the upstream `text/event-stream` body.
///
/// Provider chunks do not align with SSE frame boundaries, so bytes are buffered until a full
/// `\n\n`-terminated frame is available.
#[derive(Debug, Default)]
pub struct SseFrameParser {
    buffer: String,
}

impl SseFrameParser {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds a raw chunk and returns every `data:` payload completed by it.
    pub fn push(&mut self, chunk: &str) -> Vec<String> {
        self.buffer.push_str(chunk);
        let mut payloads = Vec::new();

        while let Some(idx) = find_frame_end(&self.buffer) {
            let (frame, rest) = self.buffer.split_at(idx.0);
            let frame = frame.to_string();
            self.buffer = rest[idx.1..].to_string();
            if let Some(payload) = data_payload(&frame) {
                payloads.push(payload);
            }
        }

        payloads
    }
}

/// Locates the end of the first complete SSE frame, returning `(frame_len, separator_len)`.
fn find_frame_end(buffer: &str) -> Option<(usize, usize)> {
    let lf = buffer.find("\n\n").map(|i| (i, 2));
    let crlf = buffer.find("\r\n\r\n").map(|i| (i, 4));
    match (lf, crlf) {
        (Some(a), Some(b)) => Some(if a.0 <= b.0 { a } else { b }),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Concatenates the `data:` lines of one SSE frame, ignoring comments and other fields.
fn data_payload(frame: &str) -> Option<String> {
    let mut data = String::new();
    for line in frame.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(rest.trim_start());
        }
    }
    if data.trim().is_empty() {
        None
    } else {
        Some(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_request(text: &str) -> ChatRequest {
        ChatRequest {
            messages: vec![ChatMessage {
                role: "user".to_string(),
                text: text.to_string(),
            }],
            context_path: None,
        }
    }

    #[test]
    fn validates_conversation_shape() {
        assert_eq!(
            validate_chat_request(&ChatRequest {
                messages: vec![],
                context_path: None
            }),
            Err(ChatRequestError::EmptyConversation)
        );
        assert_eq!(
            validate_chat_request(&user_request("   ")),
            Err(ChatRequestError::EmptyPrompt)
        );
        assert_eq!(validate_chat_request(&user_request("hello")), Ok(()));
    }

    #[test]
    fn maps_assistant_role_to_gemini_model_role() {
        let request = ChatRequest {
            messages: vec![
                ChatMessage {
                    role: "assistant".to_string(),
                    text: "previous answer".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    text: "follow up".to_string(),
                },
            ],
            context_path: None,
        };
        let payload = build_gemini_payload(&request, None);
        assert_eq!(payload["contents"][0]["role"], "model");
        assert_eq!(payload["contents"][1]["role"], "user");
    }

    #[test]
    fn binds_document_context_into_system_instruction() {
        let instruction = build_system_instruction(Some("product_vision.md"), Some("# Vision"));
        assert!(instruction.contains("product_vision.md"));
        assert!(instruction.contains("# Vision"));
    }

    #[test]
    fn truncates_oversized_document_context() {
        let huge = "x".repeat(MAX_CONTEXT_CHARS + 500);
        let excerpt = truncate_context(&huge);
        assert!(excerpt.contains("document truncated"));
        assert!(excerpt.chars().count() < huge.chars().count() + 100);
    }

    #[test]
    fn extracts_token_text_and_skips_metadata_chunks() {
        let chunk = serde_json::json!({
            "candidates": [{ "content": { "parts": [{ "text": "Hello" }, { "text": " world" }] } }]
        });
        assert_eq!(
            extract_text_from_gemini_chunk(&chunk),
            Some("Hello world".to_string())
        );

        let metadata = serde_json::json!({ "usageMetadata": { "totalTokenCount": 7 } });
        assert_eq!(extract_text_from_gemini_chunk(&metadata), None);
    }

    #[test]
    fn reassembles_frames_split_across_chunks() {
        let mut parser = SseFrameParser::new();
        assert!(parser.push("data: {\"a\":").is_empty());
        let payloads = parser.push("1}\n\ndata: {\"b\":2}\n\n");
        assert_eq!(
            payloads,
            vec!["{\"a\":1}".to_string(), "{\"b\":2}".to_string()]
        );
    }

    #[test]
    fn builds_stream_endpoint_without_double_slash() {
        let config = CopilotConfig {
            base_url: "http://127.0.0.1:8080/".to_string(),
            default_model: DEFAULT_MODEL.to_string(),
        };
        assert_eq!(
            config.stream_endpoint("gemini-2.5-flash"),
            "http://127.0.0.1:8080/v1beta/models/gemini-2.5-flash:streamGenerateContent?alt=sse"
        );
    }
}
