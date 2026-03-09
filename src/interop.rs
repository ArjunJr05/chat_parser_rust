#[cfg(not(target_arch = "wasm32"))]
use std::ffi::CStr;
use crate::com::zoho::arattai::core::messages::WhatsAppExport as ProtoExport;
use crate::com::zoho::arattai::core::whats_app_parse::WhatsAppChatParser;
use prost::Message;
#[cfg(not(target_arch = "wasm32"))]
use libc::{c_char, size_t};
use wasm_bindgen::prelude::*;

#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::objects::{JClass, JString};
#[cfg(target_os = "android")]
use jni::sys::jbyteArray;

#[cfg(not(target_arch = "wasm32"))]
#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut u8,
    pub len: size_t,
}

/// C-FFI Bridge for iOS and Desktop.
/// Returns a serialized Protobuf buffer inside a C-compatible struct.
#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub extern "C" fn parse_chat_ffi(path: *const c_char) -> ByteBuffer {
    if path.is_null() { return ByteBuffer { data: std::ptr::null_mut(), len: 0 }; }
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() { Ok(s) => s, Err(_) => return ByteBuffer { data: std::ptr::null_mut(), len: 0 } };
    match WhatsAppChatParser::parse(path_str) {
        Ok(export) => {
            let proto_export = ProtoExport { chat_name: export.get_chat_name().to_string(), messages: export.get_all_messages().to_vec() };
            let mut buf = Vec::new();
            if proto_export.encode(&mut buf).is_ok() {
                let len = buf.len();
                let data = Box::into_raw(buf.into_boxed_slice()) as *mut u8;
                ByteBuffer { data, len }
            } else { ByteBuffer { data: std::ptr::null_mut(), len: 0 } }
        }
        Err(_) => ByteBuffer { data: std::ptr::null_mut(), len: 0 },
    }
}

/// Frees the memory allocated by `parse_chat_ffi`.
#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() { unsafe { let _ = Box::from_raw(std::slice::from_raw_parts_mut(buffer.data, buffer.len)); } }
}

/// WASM Bridge for Web.
/// Takes a byte slice and returns a Vector of bytes (Protobuf).
#[wasm_bindgen]
pub fn parse_chat_wasm(zip_bytes: &[u8]) -> Vec<u8> {
    match WhatsAppChatParser::parse_bytes(zip_bytes) {
        Ok(export) => {
            let proto_export = ProtoExport { chat_name: export.get_chat_name().to_string(), messages: export.get_all_messages().to_vec() };
            let mut buf = Vec::new();
            if proto_export.encode(&mut buf).is_ok() { buf } else { Vec::new() }
        }
        Err(_) => Vec::new(),
    }
}

/// JNI bridge for Android. 
/// The function name MUST match the package and class name in Kotlin.
/// Kotlin: package com.example.imported_rust -> class WhatsAppAndroidConnector -> external fun parseChatNative
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_example_imported_1rust_WhatsAppAndroidConnector_parseChatNative(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jbyteArray {
    // 1. Get the path string from JNI
    let path_str: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    // 2. Call the parser (same logic as FFI)
    match WhatsAppChatParser::parse(&path_str) {
        Ok(export) => {
            let proto_export = ProtoExport { 
                chat_name: export.get_chat_name().to_string(), 
                messages: export.get_all_messages().to_vec() 
            };
            let mut buf = Vec::new();
            if proto_export.encode(&mut buf).is_ok() {
                // 3. Convert Rust Vec<u8> to JNI jbyteArray raw pointer
                match env.byte_array_from_slice(&buf) {
                    Ok(arr) => arr.as_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}