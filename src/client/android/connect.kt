package com.zoho.arattai.client

import android.util.Log
import com.zoho.arattai.core.messages.WhatsAppExport // Pre-generated using protoc
import java.io.File

class WhatsAppAndroidConnector {

    init {
        // 1. Load the shared library (.so)
        System.loadLibrary("rust_core")
    }

    // 2. Call the Rust FFI function (Requires JNI mapping in Rust)
    private external fun parseChatNative(path: String): ByteArray?

    fun handleChatImport(zipPath: String) {
        Log.d("WhatsAppParser", "Starting Rust Engine...")

        // 3. Get Serialized Output (Binary Protobuf)
        val protoBytes = parseChatNative(zipPath)

        if (protoBytes != null) {
            try {
                // 4. DESERIALIZATION (Turning symbols into text)
                // This is the Android version of what decodeWhatsAppExport does in Vue
                val export = WhatsAppExport.parseFrom(protoBytes)

                // 5. Use the data in your Platform UI
                Log.i("WhatsAppParser", "Successfully imported: ${export.chatName}")
                
                for (message in export.messagesList) {
                    val sender = when {
                        message.hasText() -> message.text.base.sender
                        message.hasImage() -> message.image.base.sender
                        else -> "System"
                    }
                    val content = when {
                        message.hasText() -> message.text.text
                        message.hasImage() -> "[Image: ${message.image.name}]"
                        else -> "Unsupported Content"
                    }
                    Log.d("WhatsAppParser", "[$sender]: $content")
                }
            } catch (e: Exception) {
                Log.e("WhatsAppParser", "Failed to deserialize Protobuf: ${e.message}")
            }
        } else {
            Log.e("WhatsAppParser", "Rust core returned null (possible parsing error)")
        }
    }
}
