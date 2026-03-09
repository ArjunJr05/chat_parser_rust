import Foundation
// Requires SwiftProtobuf library in your Xcode project
// Generated using: protoc --swift_out=. messages.proto

class WhatsAppiOSImporter {
    
    func startImport(fromPath path: String) {
        print("🚀 iOS Client connecting to Rust Core via FFI...")
        
        // 1. Convert Swift string to C-string for Rust
        let cPath = (path as NSString).utf8String
        
        // 2. Call Rust FFI (returns our ByteBuffer struct)
        let buffer = parse_chat_ffi(cPath)
        
        // Ensure memory is freed in Rust once we copy it
        defer {
            free_byte_buffer(buffer)
        }
        
        if let rawData = buffer.data, buffer.len > 0 {
            // 3. Wrap raw bytes into a Swift Data object
            let binaryData = Data(bytes: rawData, count: Int(buffer.len))
            
            do {
                // 4. DESERIALIZATION (The IOS version of decodeWhatsAppExport)
                // This turns the "weird symbols" into a Swift Object
                let export = try Whatsapp_WhatsAppExport(serializedData: binaryData)
                
                // 5. Display on Platform
                print("✅ Imported Chat: \(export.chatName)")
                
                for message in export.messages {
                    if message.hasText {
                         print("[\(message.text.base.sender)]: \(message.text.text)")
                    } else if message.hasImage {
                         print("[\(message.image.base.sender)]: Sent an Image (\(message.image.name))")
                    }
                }
                
            } catch {
                print("❌ Protobuf Deserialization Failed: \(error)")
            }
        } else {
            print("❌ Nothing returned from Rust Core")
        }
    }
}
