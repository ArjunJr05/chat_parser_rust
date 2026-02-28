use prost::Enumeration;
use prost::Message;
use prost_types::Timestamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
#[repr(i32)]
pub enum Type {
    Text = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
    Document = 4,
    Sticker = 5,
}

#[derive(Clone, PartialEq, Message)]
pub struct MessageBase {
    #[prost(string, tag = "1")]
    pub sender: String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<Timestamp>,
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub text: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ImageMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(uint32, tag = "3")]
    pub height: u32,
    #[prost(uint32, tag = "4")]
    pub width: u32,
    #[prost(uint64, tag = "5")]
    pub size: u64,
    #[prost(string, tag = "6")]
    pub extension: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct VideoMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(uint64, tag = "3")]
    pub size: u64,
    #[prost(string, tag = "4")]
    pub duration: String,
    #[prost(string, tag = "5")]
    pub extension: String,
    #[prost(uint32, tag = "6")]
    pub width: u32,
    #[prost(uint32, tag = "7")]
    pub height: u32,
}

#[derive(Clone, PartialEq, Message)]
pub struct AudioMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(uint64, tag = "3")]
    pub size: u64,
    #[prost(string, tag = "4")]
    pub duration: String,
    #[prost(string, tag = "5")]
    pub extension: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct DocumentMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub extension: String,
    #[prost(uint64, tag = "4")]
    pub size: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct StickerMessage {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<MessageBase>,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(uint64, tag = "3")]
    pub size: u64,
    #[prost(string, tag = "4")]
    pub extension: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct WhatsAppMessage {
    #[prost(oneof = "whatsapp_message::Content", tags = "1, 2, 3, 4, 5, 6")]
    pub content: ::core::option::Option<whatsapp_message::Content>,
}

pub mod whatsapp_message {
    use super::*;
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Text(TextMessage),
        #[prost(message, tag = "2")]
        Image(ImageMessage),
        #[prost(message, tag = "3")]
        Video(VideoMessage),
        #[prost(message, tag = "4")]
        Audio(AudioMessage),
        #[prost(message, tag = "5")]
        Document(DocumentMessage),
        #[prost(message, tag = "6")]
        Sticker(StickerMessage),
    }
}

pub use whatsapp_message::Content;

#[derive(Clone, PartialEq, Message)]
pub struct WhatsAppExport {
    #[prost(string, tag = "1")]
    pub chat_name: String,
    #[prost(message, repeated, tag = "2")]
    pub messages: Vec<WhatsAppMessage>,
}
