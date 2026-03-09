package com.example.imported_rust

import android.util.Log
import whatsapp.WhatsAppExport // Pre-generated using protoc
import java.io.File

class WhatsAppAndroidConnector {

    init {
        // 1. Load the shared library (.so)
        System.loadLibrary("rust_core")
    }

    // 2. Call the Rust FFI function (Requires JNI mapping in Rust)
    private external fun parseChatNative(path: String): ByteArray?

    fun parseChatAndGetProtoBytes(zipPath: String): ByteArray? {
        Log.d("WhatsAppParser", "Starting Rust Engine for file: $zipPath")

        // 3. Get Serialized Output (Binary Protobuf)
        val protoBytes = parseChatNative(zipPath)

        if (protoBytes == null) {
            Log.e("WhatsAppParser", "Rust core returned null (possible package name mismatch or file error)")
        }
        
        return protoBytes
    }
}
