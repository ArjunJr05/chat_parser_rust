use crate::com::zoho::arattai::core::messages::MessageBase;

/// Represents a video clip attachment parsed from a WhatsApp chat export.
#[derive(Debug, Clone)]
pub struct VideoMessage {
    pub base: MessageBase,
    /// The filename of the video file as stored inside the export ZIP.
    pub name: String,
    /// The uncompressed file size of the video in bytes.
    pub size: u64,
    /// The playback duration of the video, formatted as "m:ss".
    pub duration: String,
    /// The lowercase file extension identifying the video container format.
    pub extension: String,
    /// The horizontal resolution of the video in pixels, or 0 if unknown.
    pub width: u32,
    /// The vertical resolution of the video in pixels, or 0 if unknown.
    pub height: u32,
}

impl VideoMessage {
    pub fn new(
        base: MessageBase,
        name: String,
        size: u64,
        duration: String,
        extension: String,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            base,
            name,
            size,
            duration,
            extension,
            width,
            height,
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

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
