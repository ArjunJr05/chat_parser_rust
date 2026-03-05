//! This module contains documentation and bridge code for the various
//! platform-specific clients that connect to the Rust core.

/// Documentation for the Android client.
/// 
/// The Android client connects via Java Native Interface (JNI). 
/// It uses the `librust_core.so` shared library.
pub mod android {}

/// Documentation for the iOS client.
/// 
/// The iOS client connects via a static C-FFI bridge.
/// It uses the `librust_core.a` static library and a C header.
pub mod ios {}

/// Documentation for the Web client.
/// 
/// The Web client connects via WebAssembly (WASM).
/// It uses the `rust_core_bg.wasm` file and JavaScript glue code.
pub mod web {}
