#ifndef RUST_CORE_H
#define RUST_CORE_H

#include <stdint.h>
#include <stddef.h>

/**
 * Matching the 'ByteBuffer' struct from interop.rs
 * This tells Swift how to read the data returned by Rust.
 */
typedef struct {
    uint8_t *data;
    size_t len;
} ByteBuffer;

/**
 * Function to parse the chat from a file path.
 * Matches: #[unsafe(no_mangle)] pub extern "C" fn parse_chat_ffi
 */
ByteBuffer parse_chat_ffi(const char *path);

/**
 * Function to free the memory allocated by Rust.
 * Matches: #[unsafe(no_mangle)] pub extern "C" fn free_byte_buffer
 */
void free_byte_buffer(ByteBuffer buffer);

#endif /* RUST_CORE_H */
