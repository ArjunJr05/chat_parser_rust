package com.example.imported_rust

import android.util.Log
import whatsapp.WhatsAppExport // Pre-generated using protoc
import java.io.File
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

class WhatsAppAndroidConnector {

    init {
        // 1. Load the shared library (.so)
        System.loadLibrary("rust_core")
    }

    // 2. Call the Rust FFI function (Requires JNI mapping in Rust)
    private external fun parseChatNative(path: String): ByteArray?

    fun parseChatAndGetProtoBytes(zipPath: String): ByteArray? {
        Log.d("WhatsAppParser", "Starting Rust Engine for file: $zipPath")
        val protoBytes = parseChatNative(zipPath)
        
        if (protoBytes != null) {
            try {
                val export = WhatsAppExport.parseFrom(protoBytes)
                Log.i("WhatsAppParser", "Parsed Chat: ${export.chatName}")
                
                val dateFormat = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())

                for (message in export.messagesList) {
                    when {
                        message.hasText() -> {
                            val textMsg = message.text
                            val base = textMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [TEXT MESSAGE] ---")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Content: ${textMsg.text}")
                        }
                        message.hasImage() -> {
                            val imgMsg = message.image
                            val base = imgMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [IMAGE MESSAGE] ---")
                            Log.d("ChatOutput", "Name: ${imgMsg.name}")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                            Log.d("ChatOutput", "Extension: ${imgMsg.extension}")
                            Log.d("ChatOutput", "Dimensions: ${imgMsg.width}x${imgMsg.height}")
                            Log.d("ChatOutput", "Size: ${imgMsg.size} bytes")
                        }
                        message.hasVideo() -> {
                            val vidMsg = message.video
                            val base = vidMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [VIDEO MESSAGE] ---")
                            Log.d("ChatOutput", "Name: ${vidMsg.name}")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                            Log.d("ChatOutput", "Extension: ${vidMsg.extension}")
                            Log.d("ChatOutput", "Duration: ${vidMsg.duration}")
                            Log.d("ChatOutput", "Size: ${vidMsg.size} bytes")
                        }
                        message.hasAudio() -> {
                            val audMsg = message.audio
                            val base = audMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [AUDIO MESSAGE] ---")
                            Log.d("ChatOutput", "Name: ${audMsg.name}")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                            Log.d("ChatOutput", "Duration: ${audMsg.duration}")
                            Log.d("ChatOutput", "Size: ${audMsg.size} bytes")
                        }
                        message.hasDocument() -> {
                            val docMsg = message.document
                            val base = docMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [DOCUMENT MESSAGE] ---")
                            Log.d("ChatOutput", "Name: ${docMsg.name}")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                            Log.d("ChatOutput", "Extension: ${docMsg.extension}")
                            Log.d("ChatOutput", "Size: ${docMsg.size} bytes")
                        }
                        message.hasSticker() -> {
                            val stickerMsg = message.sticker
                            val base = stickerMsg.base
                            val dateStr = dateFormat.format(Date(base.timestamp.seconds * 1000))
                            
                            Log.d("ChatOutput", "--- [STICKER MESSAGE] ---")
                            Log.d("ChatOutput", "Sender: ${base.sender}")
                            Log.d("ChatOutput", "Type: ${base.type}")
                            Log.d("ChatOutput", "Timestamp: $dateStr")
                        }
                    }
                }
            } catch (e: Exception) {
                Log.e("WhatsAppParser", "Error reading data: ${e.message}")
            }
        }
        
        return protoBytes
    }
}
