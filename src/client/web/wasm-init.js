import init, { parse_chat_wasm } from '../../../pkg/rust_core.js';
import protobuf from 'protobufjs';

let wasmReady = false;
let root = null;

export async function initWasm() {
    if (wasmReady) return;

    // Initialize WASM
    await init();

    // Load Protobuf Definitions
    // Note: In a real app, you might serve this from your public assets
    root = await protobuf.load('/proto/messages.proto');

    wasmReady = true;
    console.log('WASM and Protobuf Initialized');
}

export function decodeWhatsAppExport(bytes) {
    if (!root) throw new Error("Protobuf not initialized");
    const WhatsAppExport = root.lookupType("whatsapp.WhatsAppExport");
    const message = WhatsAppExport.decode(bytes);
    return WhatsAppExport.toObject(message, {
        longs: String,
        enums: String,
        bytes: String,
        defaults: true,
        arrays: true,
        objects: true,
        oneofs: true
    });
}

export { parse_chat_wasm };
