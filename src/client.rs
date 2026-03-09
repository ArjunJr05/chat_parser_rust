//! This module contains documentation and bridge code for the various
//! platform-specific clients that connect to the Rust core.
//! 
//! # Integration Examples
//! 
//! Because the client code lives in non-Rust files (.kt, .swift, .vue), 
//! the source code is provided inside the documentation blocks below.
//! 
//! # Downloads
//! 
//! Download the complete ready-to-use starter package for your platform.
//! Each ZIP contains **everything** needed — the connector code, the compiled
//! Rust binary, and the Protobuf schema.
//! 
//! | Platform | Download | Contains |
//! |----------|----------|----------|
//! | Android | [Download android_client.zip](https://arjunjr05.github.io/chat_parser_rust/rust_core/android_client.zip) | `connect.kt` + `librust_core.so` (arm64, armv7, x86_64) + `messages.proto` |
//! | iOS | [Download ios_client.zip](https://arjunjr05.github.io/chat_parser_rust/rust_core/ios_client.zip) | `connect.swift` + `rust_core.h` + `librust_core.a` + `messages.proto` |
//! | Web | [Download web_client.zip](https://arjunjr05.github.io/chat_parser_rust/rust_core/web_client.zip) | `connect.vue` + `wasm-init.js` + `rust_core_bg.wasm` + `rust_core.js` + `messages.proto` |
//!
//! ## How to use the files
//!
//! ### Android (`.so` file)
//! 1. Unzip and place the `jniLibs/` folder into your Android project's `app/src/main/` directory.
//! 2. Place `connect.kt` in your Kotlin source folder.
//! 3. Run `protoc` on `messages.proto` to generate the Java Protobuf classes.
//!
//! ### iOS (`.a` file)
//! 1. Drag `librust_core.a` into your Xcode project → Link Binary with Libraries.
//! 2. Add `rust_core.h` to your **Bridging Header**.
//! 3. Run `protoc --swift_out=.` on `messages.proto` to generate Swift Protobuf classes.
//!
//! ### Web (`.wasm` file)
//! 1. Place `rust_core_bg.wasm` and `rust_core.js` in your project's `public/wasm/` folder.
//! 2. Place `messages.proto` in your `public/proto/` folder.
//! 3. Install dependencies: `npm install vite vite-plugin-wasm protobufjs`
//! 4. Use `connect.vue` and `wasm-init.js` as the entry point.

/// Android client integration (Kotlin).
/// 
/// The Android client connects via JNI using the `.so` shared library.
/// 
/// [**Download Android Starter ZIP**](https://arjunjr05.github.io/chat_parser_rust/rust_core/android_client.zip)
/// 
/// **ZIP Contents:**
/// ```text
/// android_client.zip
/// ├── connect.kt                          ← Copy to your Kotlin source folder
/// ├── messages.proto                      ← Run protoc to generate Java classes
/// └── jniLibs/
///     ├── arm64-v8a/librust_core.so        ← Modern phones (most common)
///     ├── armeabi-v7a/librust_core.so      ← Older 32-bit phones
///     └── x86_64/librust_core.so           ← Emulators
/// ```
/// 
/// ### How to connect (Kotlin):
/// ```kotlin
/// package com.zoho.arattai
/// 
/// import android.util.Log
/// import whatsapp.WhatsAppExport // The package name depends on your messages.proto
/// 
/// class WhatsAppAndroidConnector {
///     init {
///         System.loadLibrary("rust_core")
///     }
/// 
///     private external fun parseChatNative(path: String): ByteArray?
/// 
///     fun parseChatAndGetProtoBytes(zipPath: String): ByteArray? {
///         return parseChatNative(zipPath)
///     }
/// }
/// ```
pub mod android {}

/// iOS client integration (Swift).
/// 
/// The iOS client connects via a C-FFI bridge using the `.a` static library.
/// 
/// [**Download iOS Starter ZIP**](https://arjunjr05.github.io/chat_parser_rust/rust_core/ios_client.zip)
/// 
/// **ZIP Contents:**
/// ```text
/// ios_client.zip
/// ├── connect.swift      ← Add to your Xcode Swift project
/// ├── rust_core.h        ← Add to your Xcode Bridging Header
/// ├── librust_core.a     ← Drag into Xcode → Link Binary With Libraries
/// └── messages.proto     ← Run: protoc --swift_out=. messages.proto
/// ```
/// 
/// ### How to connect (Swift):
/// ```swift
/// import Foundation
/// 
/// class WhatsAppiOSImporter {
///     func startImport(fromPath path: String) {
///         let cPath = (path as NSString).utf8String
///         // parse_chat_ffi is exposed from librust_core.a via rust_core.h
///         let buffer = parse_chat_ffi(cPath)
///         defer { free_byte_buffer(buffer) }
/// 
///         if let rawData = buffer.data, buffer.len > 0 {
///             let binaryData = Data(bytes: rawData, count: Int(buffer.len))
///             let export = try Whatsapp_WhatsAppExport(serializedData: binaryData)
///             print("Imported: \(export.chatName)")
///             for msg in export.messages where msg.hasText {
///                 print("[\(msg.text.base.sender)]: \(msg.text.text_p)")
///             }
///         }
///     }
/// }
/// ```
pub mod ios {}

/// Web client integration (Vue + WASM).
/// 
/// The Web client loads the Rust engine in the browser using the `.wasm` file.
/// 
/// [**Download Web Starter ZIP**](https://arjunjr05.github.io/chat_parser_rust/rust_core/web_client.zip)
/// 
/// **ZIP Contents:**
/// ```text
/// web_client.zip
/// ├── connect.vue          ← The main Vue component (UI + logic)
/// ├── wasm-init.js         ← Loads WASM and Protobuf (the bridge)
/// ├── vite.config.js       ← Required Vite config for WASM support
/// ├── rust_core_bg.wasm    ← The compiled Rust engine (put in public/wasm/)
/// ├── rust_core.js         ← JS glue for WASM (put in public/wasm/)
/// └── messages.proto       ← Protobuf schema (put in public/proto/)
/// ```
/// 
/// ### How to connect (JavaScript):
/// ```javascript
/// // wasm-init.js — loads the .wasm engine and the .proto schema
/// import init, { parse_chat_wasm } from './rust_core.js';
/// import protobuf from 'protobufjs';
/// 
/// let root = null;
/// 
/// export async function initWasm() {
///     await init(); // Loads rust_core_bg.wasm
///     root = await protobuf.load('/proto/messages.proto');
/// }
/// 
/// export function decodeWhatsAppExport(bytes) {
///     const WhatsAppExport = root.lookupType("whatsapp.WhatsAppExport");
///     return WhatsAppExport.toObject(WhatsAppExport.decode(bytes), {
///         longs: String, enums: String, defaults: true, arrays: true
///     });
/// }
/// 
/// export { parse_chat_wasm };
/// ```
pub mod web {}
