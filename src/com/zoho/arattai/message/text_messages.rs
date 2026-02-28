use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents a plain-text message parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct TextMessage {
    pub base: MessageBase,
    /// The raw text content of the message exactly as it appears in the WhatsApp transcript.
    pub text: String,
}

impl TextMessage {
    pub fn new(base: MessageBase, text: String) -> Self {
        Self { base, text }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}
