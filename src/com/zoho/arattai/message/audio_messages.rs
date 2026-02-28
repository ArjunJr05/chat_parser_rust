use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents a voice note or audio file attachment parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct AudioMessage {
    pub base: MessageBase,
    /// The filename of the audio file as stored inside the export ZIP.
    pub name: String,
    /// The uncompressed file size of the audio in bytes.
    pub size: u64,
    /// The playback duration of the audio clip, formatted as "m:ss".
    pub duration: String,
    /// The lowercase file extension identifying the audio format.
    pub extension: String,
}

impl AudioMessage {
    pub fn new(
        base: MessageBase,
        name: String,
        size: u64,
        duration: String,
        extension: String,
    ) -> Self {
        Self {
            base,
            name,
            size,
            duration,
            extension,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_duration(&self) -> &str {
        &self.duration
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}
