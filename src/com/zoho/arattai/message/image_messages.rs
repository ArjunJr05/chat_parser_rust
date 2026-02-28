use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents a photo or image attachment parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct ImageMessage {
    pub base: MessageBase,
    /// The filename of the image as stored inside the export ZIP.
    pub name: String,
    /// The vertical dimension of the image in pixels, or 0 if unknown.
    pub height: u32,
    /// The horizontal dimension of the image in pixels, or 0 if unknown.
    pub width: u32,
    /// The uncompressed file size of the image in bytes.
    pub size: u64,
    /// The lowercase file extension that identifies the image format.
    pub extension: String,
}

impl ImageMessage {
    pub fn new(
        base: MessageBase,
        name: String,
        height: u32,
        width: u32,
        size: u64,
        extension: String,
    ) -> Self {
        Self {
            base,
            name,
            height,
            width,
            size,
            extension,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}
