use crate::com::zoho::arattai::core::messages::WhatsAppMessage;

/// Immutable container for a fully-parsed WhatsApp chat export.
///
/// A `WhatsAppExport` instance is created by `WhatsAppChatParser::parse`
/// after it has read the export ZIP, discovered the chat transcript,
/// and converted every line into a typed Message.
#[derive(Debug, Clone)]
pub struct WhatsAppExport {
    /// The human-readable chat name derived from the export ZIP filename.
    chat_name: String,

    /// All messages in chronological order exactly as they appear
    /// in the WhatsApp transcript file.
    messages: Vec<WhatsAppMessage>,
}

impl WhatsAppExport {
    /// Creates a new `WhatsAppExport`.
    pub fn new(chat_name: String, messages: Vec<WhatsAppMessage>) -> Self {
        Self {
            chat_name,
            messages,
        }
    }

    /// Returns the complete, ordered list of all messages in this export.
    pub fn get_all_messages(&self) -> &[WhatsAppMessage] {
        &self.messages
    }

    /// Returns the display name of the chat.
    pub fn get_chat_name(&self) -> &str {
        &self.chat_name
    }
}
