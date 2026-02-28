use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents a generic file attachment message parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct DocumentMessage {
    pub base: MessageBase,
    /// The filename of the document as stored inside the export ZIP.
    pub name: String,
    /// The lowercase file extension identifying the document format.
    pub extension: String,
    /// The uncompressed file size of the document in bytes.
    pub size: u64,
}

impl DocumentMessage {
    pub fn new(base: MessageBase, name: String, extension: String, size: u64) -> Self {
        Self {
            base,
            name,
            extension,
            size,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}
