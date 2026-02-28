#[cfg(not(target_arch = "wasm32"))]
use std::ffi::CStr;
use crate::com::zoho::arattai::core::messages::WhatsAppExport as ProtoExport;
use crate::com::zoho::arattai::core::whats_app_parse::WhatsAppChatParser;
use prost::Message;
#[cfg(not(target_arch = "wasm32"))]
use libc::{c_char, size_t};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut u8,
    pub len: size_t,
}

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

#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() { unsafe { let _ = Box::from_raw(std::slice::from_raw_parts_mut(buffer.data, buffer.len)); } }
}

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