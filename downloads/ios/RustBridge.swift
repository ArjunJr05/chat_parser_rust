// Create a Swift FFI bridge to the Rust static library (.a).
// This bridge uses direct C-symbol mapping to avoid Bridging Header issues.

import Foundation

// MARK: - Direct C FFI Declarations
// These map directly to the symbols in librust_core.a

struct ByteBuffer {
    var data: UnsafeMutablePointer<UInt8>?
    var len: Int
}

@_silgen_name("parse_chat_ffi")
private func rust_parse_chat_ffi(_ path: UnsafePointer<CChar>) -> ByteBuffer

@_silgen_name("free_byte_buffer")
private func rust_free_byte_buffer(_ buffer: ByteBuffer)

enum RustBridgeError: Error, LocalizedError {
    case engineFailed
    case engineUnavailable

    var errorDescription: String? {
        switch self {
        case .engineFailed: return "Rust engine failed to parse chat"
        case .engineUnavailable: return "Rust engine symbol unavailable"
        }
    }
}

final class RustBridge {
    static let shared = RustBridge()
    private init() {}

    func parseWhatsAppZip(zipBytes: Data) async throws -> Data {
        // Call into Rust on a background thread
        return try await withCheckedThrowingContinuation { continuation in
            DispatchQueue.global(qos: .userInitiated).async {
                // Since the C API parse_chat_ffi takes a path (const char *), 
                // we need to write the ZIP data to a temporary file first.
                let tempDir = FileManager.default.temporaryDirectory
                let tempFile = tempDir.appendingPathComponent(UUID().uuidString + ".zip")
                
                do {
                    try zipBytes.write(to: tempFile)
                    
                    let path = tempFile.path
                    
                    // Call the Rust function mapped via @_silgen_name
                    let byteBuffer = rust_parse_chat_ffi(path)
                    
                    // Clean up temp file
                    try? FileManager.default.removeItem(at: tempFile)
                    
                    if let dataPtr = byteBuffer.data, byteBuffer.len > 0 {
                        // Copy bytes into Swift Data
                        let data = Data(bytes: dataPtr, count: Int(byteBuffer.len))
                        
                        // Free the memory allocated by Rust
                        rust_free_byte_buffer(byteBuffer)
                        
                        continuation.resume(returning: data)
                    } else {
                        continuation.resume(throwing: RustBridgeError.engineFailed)
                    }
                } catch {
                    continuation.resume(throwing: error)
                }
            }
        }
    }
}
