use serde::{Deserialize, Serialize};

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;

// Models API
const MODELS_LIST: &str = "models";
const MODELS_RETRIEVE: &str = "models/";
// Completions API
const COMPLETION_CREATE: &str = "completions";
// Chat API
const CHAT_COMPLETION_CREATE: &str = "chat/completions";
// Edits API
const EDIT_CREATE: &str = "edits";
// Embeddings API
const EMBEDDINGS_CREATE: &str = "embeddings";

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
	pub prompt_tokens: Option<u32>,
	pub completion_tokens: Option<u32>,
	pub total_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
	pub text: Option<String>,
	pub index: u32,
	pub logprobs: Option<String>,
	pub finish_reason: Option<String>,
	pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	pub role: Role,
	pub content: String,
}

impl Clone for Message {
	fn clone(&self) -> Self {
		Self { role: self.role.clone(), content: self.content.clone() }
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	System,
	Assistant,
	User,
}

impl Clone for Role {
	fn clone(&self) -> Self {
		match self {
			Self::System => Self::System,
			Self::Assistant => Self::Assistant,
			Self::User => Self::User,
		}
	}
}
