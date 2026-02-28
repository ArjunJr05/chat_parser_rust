use crate::com::zoho::arattai::core::messages::{
    whatsapp_message, AudioMessage, DocumentMessage, ImageMessage, MessageBase, StickerMessage,
    TextMessage, Type, VideoMessage, WhatsAppMessage,
};
use crate::com::zoho::arattai::core::whats_app_export::WhatsAppExport;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Seek, SeekFrom};
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use tempfile::NamedTempFile;

use zip::ZipArchive;

enum DataSource<'a> {
    ZipCursor(ZipArchive<io::Cursor<&'a [u8]>>),
    #[cfg(not(target_arch = "wasm32"))]
    ZipFile(ZipArchive<File>),
    #[cfg(not(target_arch = "wasm32"))]
    Dir(PathBuf),
}

impl<'a> DataSource<'a> {
    #[cfg(not(target_arch = "wasm32"))]
    fn read_to_vec(&mut self, name: &str) -> io::Result<Vec<u8>> {
        match self {
            Self::ZipCursor(archive) => {
                let mut entry = archive.by_name(name).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
                let mut buf = Vec::new();
                entry.read_to_end(&mut buf)?;
                Ok(buf)
            }
            Self::ZipFile(archive) => {
                let mut entry = archive.by_name(name).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
                let mut buf = Vec::new();
                entry.read_to_end(&mut buf)?;
                Ok(buf)
            }
            Self::Dir(path) => {
                let file_path = path.join(name);
                std::fs::read(file_path)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_file_handle(&mut self, name: &str, tmp: &mut Option<NamedTempFile>) -> io::Result<File> {
        match self {
            Self::ZipCursor(archive) => {
                let mut entry = archive.by_name(name).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
                let t = NamedTempFile::new()?;
                let mut file = t.as_file().try_clone()?;
                io::copy(&mut entry, &mut file)?;
                *tmp = Some(t);
                Ok(file)
            }
            Self::ZipFile(archive) => {
                let mut entry = archive.by_name(name).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
                let t = NamedTempFile::new()?;
                let mut file = t.as_file().try_clone()?;
                io::copy(&mut entry, &mut file)?;
                *tmp = Some(t);
                Ok(file)
            }
            Self::Dir(path) => {
                let file_path = path.join(name);
                File::open(file_path)
            }
        }
    }
}

struct MediaEntry {
    name: String,
    size: u64,
}

pub struct WhatsAppChatParser;

impl WhatsAppChatParser {
    const DATE_PATTERN: &'static str = "%d/%m/%Y, %I:%M %p";

    pub fn parse_bytes(zip_bytes: &[u8]) -> Result<WhatsAppExport, Box<dyn std::error::Error>> {
        let chat_name = "WhatsApp Chat".to_string();
        let cursor = io::Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(cursor)?;
        
        let mut media_files = HashMap::new();
        let mut transcript_entry_name = None;

        for i in 0..archive.len() {
            let entry = archive.by_index(i)?;
            if entry.is_dir() { continue; }
            let name = entry.name().to_string();
            if name.ends_with(".txt") {
                transcript_entry_name = Some(name.clone());
            } else {
                media_files.insert(
                    name.clone(),
                    MediaEntry { name: name.clone(), size: entry.size() },
                );
            }
        }

        let mut source = DataSource::ZipCursor(archive);
        let mut messages = Vec::new();
        if let Some(txt_name) = transcript_entry_name {
            let transcript = match &mut source {
                DataSource::ZipCursor(archive) => {
                    let mut entry = archive.by_name(&txt_name)?;
                    let mut s = String::new();
                    entry.read_to_string(&mut s)?;
                    s
                }
                #[cfg(not(target_arch = "wasm32"))]
                _ => unreachable!(),
            };
            let reader = BufReader::new(transcript.as_bytes());
            messages = Self::parse_transcript(reader, &media_files, &mut source)?;
        }

        Ok(WhatsAppExport::new(chat_name, messages))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<WhatsAppExport, Box<dyn std::error::Error>> {
        let p = path.as_ref();
        if p.is_dir() {
            let chat_name = Self::extract_chat_name(p);
            let mut media_files = HashMap::new();
            let mut transcript_entry_name = None;
            for entry in std::fs::read_dir(p)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".txt") {
                    transcript_entry_name = Some(name.clone());
                } else if entry.file_type()?.is_file() {
                    media_files.insert(
                        name.clone(),
                        MediaEntry {
                            name: name.clone(),
                            size: entry.metadata()?.len(),
                        },
                    );
                }
            }
            let mut source = DataSource::Dir(p.to_path_buf());
            let mut messages = Vec::new();
            if let Some(txt_name) = transcript_entry_name {
                let transcript = std::fs::read_to_string(p.join(&txt_name))?;
                let reader = BufReader::new(transcript.as_bytes());
                messages = Self::parse_transcript(reader, &media_files, &mut source)?;
            }
            return Ok(WhatsAppExport::new(chat_name, messages));
        } else {
            let chat_name = Self::extract_chat_name(p);
            let file = File::open(p)?;
            let mut archive = ZipArchive::new(file)?;
            let mut media_files = HashMap::new();
            let mut transcript_entry_name = None;
            for i in 0..archive.len() {
                let entry = archive.by_index(i)?;
                let name = entry.name().to_string();
                if name.ends_with(".txt") { transcript_entry_name = Some(name); }
                else { media_files.insert(name.clone(), MediaEntry { name, size: entry.size() }); }
            }
            let mut source = DataSource::ZipFile(archive);
            let mut messages = Vec::new();
            if let Some(txt_name) = transcript_entry_name {
                let mut transcript = String::new();
                if let DataSource::ZipFile(ref mut a) = source {
                    a.by_name(&txt_name)?.read_to_string(&mut transcript)?;
                }
                let reader = BufReader::new(transcript.as_bytes());
                messages = Self::parse_transcript(reader, &media_files, &mut source)?;
            }
            Ok(WhatsAppExport::new(chat_name, messages))
        }
    }

    fn parse_transcript<R: BufRead>(
        reader: R,
        media_files: &HashMap<String, MediaEntry>,
        source: &mut DataSource,
    ) -> Result<Vec<WhatsAppMessage>, Box<dyn std::error::Error>> {
        let pattern = Regex::new(
            r"(?i)^(\d{1,2}/\d{1,2}/\d{4},[\s\u{202f}\u{00a0}]+\d{1,2}:\d{2}(?::\d{2})?[\s\u{202f}\u{00a0}]*[ap]m)\s*-\s*([^:]+):\s(.*)$",
        )?;
        let mut messages = Vec::new();
        let mut pending = String::new();
        for line_result in reader.lines() {
            let line = line_result?;
            if pattern.is_match(&line) {
                if !pending.is_empty() {
                    if let Some(msg) = Self::build_message(&pending, media_files, source) {
                        messages.push(msg);
                    }
                }
                pending = line;
            } else if !pending.is_empty() {
                pending.push('\n');
                pending.push_str(&line);
            }
        }
        if !pending.is_empty() {
            if let Some(msg) = Self::build_message(&pending, media_files, source) {
                messages.push(msg);
            }
        }
        Ok(messages)
    }

    fn to_proto_timestamp(dt: DateTime<Utc>) -> prost_types::Timestamp {
        prost_types::Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }

    fn build_message(
        raw_line: &str,
        media_files: &HashMap<String, MediaEntry>,
        source: &mut DataSource,
    ) -> Option<WhatsAppMessage> {
        let first_line = raw_line.lines().next()?;
        let pattern = Regex::new(
            r"(?i)^(\d{1,2}/\d{1,2}/\d{4},[\s\u{202f}\u{00a0}]+\d{1,2}:\d{2}(?::\d{2})?[\s\u{202f}\u{00a0}]*[ap]m)\s*-\s*([^:]+):\s(.*)$",
        ).ok()?;
        let caps = pattern.captures(first_line)?;
        let timestamp_str = caps.get(1)?.as_str();
        let sender = caps.get(2)?.as_str().trim().to_string();
        let content = caps.get(3)?.as_str().to_string();
        let timestamp = Self::parse_timestamp(timestamp_str);
        let msg_type = Self::classify_message(&content);
        let base = Some(MessageBase {
            sender,
            timestamp: Some(Self::to_proto_timestamp(timestamp)),
            r#type: msg_type as i32,
        });

        match msg_type {
            Type::Text => Some(WhatsAppMessage {
                content: Some(whatsapp_message::Content::Text(TextMessage { base, text: content })),
            }),
            Type::Image => {
                let info = Self::find_media(&content, media_files, "image");
                let name = info.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| "image.jpg".to_string());
                let size = info.as_ref().map(|i| i.size).unwrap_or(0);
                let width;
                let height;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let mut w = 0;
                    let mut h = 0;
                    if let Some(entry_info) = info {
                        if let Ok(buffer) = source.read_to_vec(&entry_info.name) {
                            if let Ok(img) = image::load_from_memory(&buffer) {
                                w = img.width();
                                h = img.height();
                            }
                        }
                    }
                    width = w;
                    height = h;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = source;
                    width = 0;
                    height = 0;
                }
                let extension = Self::extension(&name);
                Some(WhatsAppMessage {
                    content: Some(whatsapp_message::Content::Image(ImageMessage {
                        base, name, height, width, size, extension,
                    })),
                })
            }
            Type::Video => {
                let info = Self::find_media(&content, media_files, "video");
                let name = info.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| "video.mp4".to_string());
                let size = info.as_ref().map(|i| i.size).unwrap_or(0);
                let width;
                let height;
                let mut duration = "0:00".to_string();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let mut w = 0;
                    let mut h = 0;
                    if let Some(entry_info) = info {
                        let mut tmp = None;
                        if let Ok(file_handle) = source.get_file_handle(&entry_info.name, &mut tmp) {
                            duration = Self::parse_mp4_duration(&file_handle);
                            let dims = Self::parse_mp4_dimensions(&file_handle);
                            w = dims.0;
                            h = dims.1;
                        }
                    }
                    width = w;
                    height = h;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = source;
                    width = 0;
                    height = 0;
                }
                let extension = Self::extension(&name);
                Some(WhatsAppMessage {
                    content: Some(whatsapp_message::Content::Video(VideoMessage {
                        base, name, size, duration, extension, width, height,
                    })),
                })
            }
            Type::Audio => {
                let info = Self::find_media(&content, media_files, "audio");
                let name = info.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| "audio.opus".to_string());
                let size = info.as_ref().map(|i| i.size).unwrap_or(0);
                let mut duration = "0:00".to_string();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Some(entry_info) = info {
                        let ext = Self::extension(&name);
                        let mut tmp = None;
                        if let Ok(file_handle) = source.get_file_handle(&entry_info.name, &mut tmp) {
                            if ext == "opus" || ext == "ogg" {
                                duration = Self::parse_opus_duration(&file_handle);
                            } else if ext == "m4a" || ext == "aac" || ext == "mp4" {
                                duration = Self::parse_mp4_duration(&file_handle);
                            }
                        }
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let _ = source;
                }
                let extension = Self::extension(&name);
                Some(WhatsAppMessage {
                    content: Some(whatsapp_message::Content::Audio(AudioMessage {
                        base, name, size, duration, extension,
                    })),
                })
            }
            Type::Document => {
                let info = Self::find_media(&content, media_files, "document");
                let name = if let Some(i) = info { i.name.clone() } else {
                    if let Some(pos) = content.find(" (file attached)") { content[..pos].trim().to_string() }
                    else { "document.pdf".to_string() }
                };
                let size = info.as_ref().map(|i| i.size).unwrap_or(0);
                let extension = Self::extension(&name);
                Some(WhatsAppMessage {
                    content: Some(whatsapp_message::Content::Document(DocumentMessage {
                        base, name, extension, size,
                    })),
                })
            }
            Type::Sticker => {
                let info = Self::find_media(&content, media_files, "sticker");
                let name = info.as_ref().map(|i| i.name.clone()).unwrap_or_else(|| "sticker.webp".to_string());
                let size = info.as_ref().map(|i| i.size).unwrap_or(0);
                let extension = Self::extension(&name);
                Some(WhatsAppMessage {
                    content: Some(whatsapp_message::Content::Sticker(StickerMessage {
                        base, name, size, extension,
                    })),
                })
            }
        }
    }

    fn classify_message(content: &str) -> Type {
        let lc = content.to_lowercase().trim().to_string();
        if lc == "<media omitted>" { return Type::Audio; }
        if lc.contains("this message was deleted") { return Type::Text; }
        if lc.contains("(file attached)") || lc.starts_with("doc-") || lc.starts_with("img-") || lc.starts_with("vid-") || lc.starts_with("ptt-") {
            if lc.contains(".webp") { return Type::Sticker; }
            if [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp"].iter().any(|&ext| lc.contains(ext)) { return Type::Image; }
            if [".mp4", ".avi", ".mov", ".mkv", ".webm"].iter().any(|&ext| lc.contains(ext)) { return Type::Video; }
            if [".mp3", ".wav", ".ogg", ".m4a", ".aac", ".opus"].iter().any(|&ext| lc.contains(ext)) { return Type::Audio; }
            return Type::Document;
        }
        Type::Text
    }

    fn find_media<'a>(content: &str, media_files: &'a HashMap<String, MediaEntry>, media_type: &str) -> Option<&'a MediaEntry> {
        let lc = content.to_lowercase();
        for (name, entry) in media_files {
            let ln = name.to_lowercase();
            let matches_type = match media_type {
                "sticker" => ln.ends_with(".webp"),
                "image" => [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp"].iter().any(|&ext| ln.ends_with(ext)),
                "video" => [".mp4", ".avi", ".mov", ".mkv", ".webm"].iter().any(|&ext| ln.ends_with(ext)),
                "audio" => [".mp3", ".wav", ".ogg", ".m4a", ".aac", ".opus"].iter().any(|&ext| ln.ends_with(ext)),
                "document" => true,
                _ => false,
            };
            if matches_type && (lc.contains(name) || lc.contains(&ln)) { return Some(entry); }
        }
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn parse_opus_duration(file: &File) -> String {
        let mut reader = BufReader::new(file);
        let len = file.metadata().map(|m| m.len()).unwrap_or(0);
        if len == 0 { return "0:00".to_string(); }
        let start = if len > 65536 { len - 65536 } else { 0 };
        let mut buffer = vec![0u8; (len - start) as usize];
        if reader.seek(SeekFrom::Start(start)).is_ok() && reader.read_exact(&mut buffer).is_ok() {
            for i in (0..(buffer.len() as isize - 4)).rev() {
                let i = i as usize;
                if buffer[i] == 0x4F && buffer[i + 1] == 0x67 && buffer[i + 2] == 0x67 && buffer[i + 3] == 0x53 && i + 13 < buffer.len() {
                    let mut g: u64 = 0;
                    for k in (0..8).rev() { g = (g << 8) | (buffer[i + 6 + k] as u64 & 0xFF); }
                    if g > 0 { let s = g / 48000; return format!("{}:{:02}", s / 60, s % 60); }
                }
            }
        }
        "0:00".to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn parse_mp4_duration(file: &File) -> String {
        let mut reader = BufReader::new(file);
        let len = file.metadata().map(|m| m.len()).unwrap_or(0);
        let mut buffer = [0u8; 65536];
        let mut pos = 0;
        while pos + 100 < len {
            if reader.seek(SeekFrom::Start(pos)).is_err() { break; }
            let bytes_read = match reader.read(&mut buffer) { Ok(0) => break, Ok(n) => n, Err(_) => break };
            for i in 0..(bytes_read.saturating_sub(100)) {
                if buffer[i] == b'm' && buffer[i + 1] == b'v' && buffer[i + 2] == b'h' && buffer[i + 3] == b'd' {
                    let version = buffer[i + 4] as usize;
                    let (ts, dur) = if version == 1 && i + 36 < bytes_read { (Self::i32(&buffer, i + 24), Self::i64(&buffer, i + 28)) } 
                    else if i + 24 < bytes_read { (Self::i32(&buffer, i + 16), Self::i32(&buffer, i + 20)) } else { continue };
                    if ts > 0 { let s = dur / ts; return format!("{}:{:02}", s / 60, s % 60); }
                }
            }
            pos += bytes_read as u64 - 100;
        }
        "0:00".to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn parse_mp4_dimensions(file: &File) -> (u32, u32) {
        let mut reader = BufReader::new(file);
        let len = file.metadata().map(|m| m.len()).unwrap_or(0);
        let mut buffer = [0u8; 65536];
        let mut pos = 0;
        while pos + 100 < len {
            if reader.seek(SeekFrom::Start(pos)).is_err() { break; }
            let bytes_read = match reader.read(&mut buffer) { Ok(0) => break, Ok(n) => n, Err(_) => break };
            for i in 0..(bytes_read.saturating_sub(100)) {
                if buffer[i] == b't' && buffer[i + 1] == b'k' && buffer[i + 2] == b'h' && buffer[i + 3] == b'd' {
                    let version = buffer[i + 4] as usize;
                    let wo = if version == 1 { i + 92 } else { i + 80 };
                    let ho = if version == 1 { i + 96 } else { i + 84 };
                    if ho + 4 <= bytes_read {
                        let w = (Self::i32(&buffer, wo) >> 16) as u32;
                        let h = (Self::i32(&buffer, ho) >> 16) as u32;
                        if w > 0 && h > 0 { return (w, h); }
                    }
                }
            }
            pos += bytes_read as u64 - 100;
        }
        (0, 0)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn i32(b: &[u8], o: usize) -> u64 { ((b[o] as u64 & 0xFF) << 24) | ((b[o + 1] as u64 & 0xFF) << 16) | ((b[o + 2] as u64 & 0xFF) << 8) | (b[o + 3] as u64 & 0xFF) }
    #[cfg(not(target_arch = "wasm32"))]
    fn i64(b: &[u8], o: usize) -> u64 { (Self::i32(b, o) << 32) | (Self::i32(b, o + 4) & 0xFFFFFFFF) }
    fn extension(f: &str) -> String { Path::new(f).extension().and_then(|ext| ext.to_str()).unwrap_or("-").to_lowercase() }
    #[cfg(not(target_arch = "wasm32"))]
    fn extract_chat_name(path: &Path) -> String { path.file_name().and_then(|n| n.to_str()).unwrap_or("").replace(".zip", "").replace("WhatsApp Chat with ", "") }
    fn parse_timestamp(raw: &str) -> DateTime<Utc> {
        let clean = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Ok(naive) = NaiveDateTime::parse_from_str(&clean, Self::DATE_PATTERN) { return DateTime::from_naive_utc_and_offset(naive, Utc); }
        Utc::now()
    }
}
