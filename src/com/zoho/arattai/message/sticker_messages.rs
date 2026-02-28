use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents an animated WebP sticker message parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct StickerMessage {
    pub base: MessageBase,
    /// The filename of the sticker file as stored inside the export ZIP.
    pub name: String,
    /// The uncompressed file size of the sticker in bytes.
    pub size: u64,
    /// The lowercase file extension of the sticker.
    pub extension: String,
}

impl StickerMessage {
    pub fn new(base: MessageBase, name: String, size: u64, extension: String) -> Self {
        Self {
            base,
            name,
            size,
            extension,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}
