import init, { parse_chat_wasm } from './wasm/rust_core.js';
import protobuf from 'protobufjs';

let wasmReady = false;
let root = null;

export async function initWasm() {
    if (wasmReady) return;

    // Initialize WASM
    await init();

    try {
        // Load Protobuf Definitions
        // Note: fetch the file first to ensure it's readable
        const response = await fetch('/proto/messages.proto');
        if (!response.ok) throw new Error(`Failed to fetch proto: ${response.statusText}`);
        const protoText = await response.text();

        root = protobuf.parse(protoText).root;

        wasmReady = true;
        console.log('WASM and Protobuf Initialized');
        console.log('Available types in root:', Object.keys(root.nested || {}));
    } catch (error) {
        console.error('Protobuf Load Error:', error);
        throw error;
    }
}

export function decodeWhatsAppExport(bytes) {
    if (!root) throw new Error("Protobuf not initialized");

    let WhatsAppExport;
    try {
        WhatsAppExport = root.lookupType("whatsapp.WhatsAppExport");
    } catch (e) {
        // Fallback: try looking it up directly if namespace nesting is different
        WhatsAppExport = root.lookupType("WhatsAppExport");
    }

    const message = WhatsAppExport.decode(bytes);
    return WhatsAppExport.toObject(message, {
        longs: Number,
        enums: String,
        bytes: String,
        defaults: true,
        arrays: true,
        objects: true,
        oneofs: true
    });
}

export { parse_chat_wasm };