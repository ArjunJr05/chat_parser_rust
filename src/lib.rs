pub mod com;
pub mod interop;

#[cfg(not(target_arch = "wasm32"))]
use crate::com::zoho::arattai::core::messages::{WhatsAppExport as ProtoExport};
use crate::com::zoho::arattai::core::messages::{whatsapp_message, Type, WhatsAppMessage};
use crate::com::zoho::arattai::core::whats_app_export::WhatsAppExport;
#[cfg(not(target_arch = "wasm32"))]
use crate::com::zoho::arattai::core::whats_app_parse::WhatsAppChatParser;
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

fn format_proto_timestamp(ts: &Option<prost_types::Timestamp>) -> String {
    if let Some(t) = ts {
        let naive = chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                chrono::NaiveDateTime::default(),
                chrono::Utc,
            ));
        naive.format("%d/%m/%Y, %I:%M:%S %P").to_string()
    } else {
        "unknown".to_string()
    }
}

/// Prints the details of a single Message to standard output.
pub fn print_message(msg: &WhatsAppMessage, chat_name: &str) {
    let content = match msg.content.as_ref() {
        Some(c) => c,
        None => return,
    };

    let (base, details) = match content {
        whatsapp_message::Content::Text(m) => (
            m.base.as_ref().unwrap(),
            format!("Text: {}", m.text),
        ),
        whatsapp_message::Content::Image(m) => (
            m.base.as_ref().unwrap(),
            format!(
                "Image Name: {}\nImage Height: {}\nImage Width: {}\nImage Size: {} bytes\nImage Extension: {}",
                m.name, m.height, m.width, m.size, m.extension
            ),
        ),
        whatsapp_message::Content::Video(m) => (
            m.base.as_ref().unwrap(),
            format!(
                "Video Name: {}\nVideo Size: {} bytes\nVideo Duration: {}\nVideoExtension: {}\nVideo Width: {}\nVideo Height: {}",
                m.name, m.size, m.duration, m.extension, m.width, m.height
            ),
        ),
        whatsapp_message::Content::Audio(m) => (
            m.base.as_ref().unwrap(),
            format!(
                "Audio Name: {}\nAudio Size: {} bytes\nAudio Duration: {}\nAudio Extension: {}",
                m.name, m.size, m.duration, m.extension
            ),
        ),
        whatsapp_message::Content::Document(m) => (
            m.base.as_ref().unwrap(),
            format!(
                "Document Name: {}\nDocument Extension: {}\nDocument Size: {} bytes",
                m.name, m.extension, m.size
            ),
        ),
        whatsapp_message::Content::Sticker(m) => (
            m.base.as_ref().unwrap(),
            format!(
                "Sticker Name: {}\nSticker Extension: {}\nSticker Size: {} bytes",
                m.name, m.extension, m.size
            ),
        ),
    };

    println!("Chat Name: {}", chat_name);
    println!("Sender: {}", base.sender);
    println!("Timestamp: {}", format_proto_timestamp(&base.timestamp));
    let type_name = Type::try_from(base.r#type).map(|t| format!("{:?}", t)).unwrap_or_else(|_| "Unknown".to_string());
    println!("Type: {}", type_name);
    println!("{}", details);
}

/// Prints every message in the export to standard output.
pub fn print_all_messages(export: &WhatsAppExport) {
    println!("\n========== COMPLETE ARRAYLIST DATA ==========");
    println!("Chat Name: {}", export.get_chat_name());
    println!("Total Messages: {}", export.get_all_messages().len());
    println!("=============================================\n");

    let messages = export.get_all_messages();
    for (i, msg) in messages.iter().enumerate() {
        println!("--- Message #{} ---", i + 1);
        print_message(msg, export.get_chat_name());
        println!("----------------------------\n");
    }

    println!("============================================\n");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================================");
    println!("  WhatsApp Chat Parser - Chat Importer");
    println!("=======================================\n");

    print!("Enter the Zip file path: ");
    io::stdout().flush()?;
    
    let mut zip_path = String::new();
    io::stdin().read_line(&mut zip_path)?;
    let zip_path_str = zip_path.trim();

    let final_path = if zip_path_str.starts_with('"') && zip_path_str.ends_with('"') {
        zip_path_str[1..zip_path_str.len() - 1].to_string()
    } else {
        zip_path_str.to_string()
    };

    println!("\nParsing chat file...");

    match WhatsAppChatParser::parse(&final_path) {
        Ok(export) => {
            if export.get_all_messages().is_empty() {
                println!("No messages found in the ZIP file.");
                return Ok(());
            }

            println!("\n========== SUMMARY ==========");
            println!("Chat Name: {}", export.get_chat_name());
            println!("Total messages parsed: {}", export.get_all_messages().len());
            println!("=============================\n");

            print_all_messages(&export);

            let proto_export = ProtoExport {
                chat_name: export.get_chat_name().to_string(),
                messages: export.get_all_messages().to_vec(),
            };

            use prost::Message;
            let mut buf = Vec::new();
            if proto_export.encode(&mut buf).is_ok() {
                let export_file = "chat_export.bin";
                if std::fs::write(export_file, &buf).is_ok() {
                    println!("\n[Protobuf] Serialized chat to {} ({} bytes)", export_file, buf.len());
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
