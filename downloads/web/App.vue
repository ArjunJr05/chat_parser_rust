<script setup>
import { ref, onMounted } from 'vue';
import { initWasm, parse_chat_wasm, decodeWhatsAppExport } from './wasm-init';

const status = ref('READY');
const fileName = ref('');
const chatData = ref(null);

onMounted(async () => {
  try {
    await initWasm();
  } catch (error) {
    status.value = 'ERROR';
    console.error(error);
  }
});

const formatTimestamp = (ts) => {
  if (!ts || !ts.seconds) return 'Unknown';
  const date = new Date(ts.seconds * 1000);
  return date.toLocaleString();
};

const getContentType = (msg) => {
  if (msg.text) return 'TEXT';
  if (msg.image) return 'IMAGE';
  if (msg.video) return 'VIDEO';
  if (msg.audio) return 'AUDIO';
  if (msg.document) return 'DOCUMENT';
  if (msg.sticker) return 'STICKER';
  return 'UNKNOWN';
};

const getBaseInfo = (msg) => {
  const type = getContentType(msg);
  return msg[type.toLowerCase()]?.base || {};
};

const handleFileUpload = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  fileName.value = file.name;
  status.value = 'PROCESSING';
  
  try {
    const arrayBuffer = await file.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);
    
    // 1. Parse ZIP with WASM
    const protoBytes = parse_chat_wasm(bytes);
    
    // 2. Deserialize Protobuf
    chatData.value = decodeWhatsAppExport(protoBytes);
    
    status.value = 'SUCCESS';
    console.log('Parsed Data:', chatData.value);
  } catch (error) {
    status.value = 'ERROR';
    console.error(error);
  }
};
</script>

<template>
  <div class="app-container">
    <header class="main-header">
      <h1>WhatsApp Parser</h1>
    </header>

    <div v-if="fileName" class="status-bar">
      Successfully parsed: {{ fileName }}
    </div>

    <main class="content">
      <!-- Upload Section -->
      <div v-if="!chatData" class="upload-section">
        <label for="zip-input" class="upload-card">
          <div class="icon">📁</div>
          <h2>{{ status === 'PROCESSING' ? 'Processing...' : 'Upload WhatsApp ZIP' }}</h2>
          <p>Select your exported chat file to begin</p>
          <input 
            type="file" 
            accept=".zip" 
            @change="handleFileUpload" 
            id="zip-input"
            hidden
          />
        </label>
      </div>

      <!-- Chat View -->
      <div v-if="chatData" class="chat-view">
        <h2 class="chat-title">Chat: {{ chatData.chat_name || chatData.chatName }}</h2>
        
        <div class="messages-list">
          <div v-for="(msg, index) in chatData.messages" :key="index" class="message-card">
            <div class="message-badge">{{ getContentType(msg) }} MESSAGE</div>
            <div class="sender-name">{{ getBaseInfo(msg).sender }}</div>
            
            <div class="message-body">
              <p v-if="msg.text" class="text-content">{{ msg.text.text }}</p>
              <div v-else-if="msg.image" class="media-placeholder">📷 Image: {{ msg.image.name }}</div>
              <div v-else-if="msg.sticker" class="media-placeholder">✨ Sticker</div>
              <div v-else-if="msg.video" class="media-placeholder">🎥 Video: {{ msg.video.name }}</div>
              <div v-else class="media-placeholder">📎 Attachment</div>
            </div>

            <div class="message-footer">
              Sent at: {{ formatTimestamp(getBaseInfo(msg).timestamp) }} • Type: {{ getContentType(msg) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="status === 'ERROR'" class="error-msg">
        ⚠️ Failed to process file. Please ensure it is a valid WhatsApp export.
      </div>
    </main>

    <!-- Floating Action Button -->
    <button v-if="chatData" class="fab" @click="() => { chatData = null; fileName = ''; status = 'READY'; }">
      +
    </button>
  </div>
</template>

<style>
:root {
  --primary-purple: #f3ebf6;
  --accent-purple: #6c5ce7;
  --text-dark: #2d3436;
  --text-muted: #636e72;
  --bg-white: #ffffff;
  --card-shadow: 0 2px 10px rgba(0,0,0,0.05);
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  background-color: var(--primary-purple);
  color: var(--text-dark);
}

.app-container {
  max-width: 600px;
  margin: 0 auto;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-header {
  padding: 20px;
  background-color: var(--bg-white);
  text-align: center;
}

.main-header h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #6a4c93;
}

.status-bar {
  padding: 12px 20px;
  font-size: 14px;
  color: var(--text-muted);
  border-bottom: 1px solid #eee;
  background-color: var(--bg-white);
  text-align: center;
}

.content {
  flex: 1;
  padding: 20px;
}

.upload-section {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 60vh;
}

.upload-card {
  background: white;
  padding: 40px;
  border-radius: 20px;
  text-align: center;
  box-shadow: var(--card-shadow);
  cursor: pointer;
  transition: transform 0.2s;
  width: 100%;
}

.upload-card:hover {
  transform: translateY(-5px);
}

.upload-card .icon {
  font-size: 48px;
  margin-bottom: 15px;
}

.chat-title {
  font-size: 20px;
  margin-bottom: 20px;
  color: var(--text-dark);
  font-weight: 500;
}

.messages-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.message-card {
  background: white;
  padding: 16px;
  border-radius: 15px;
  box-shadow: var(--card-shadow);
  text-align: left;
}

.message-badge {
  font-size: 10px;
  font-weight: 700;
  color: #a29bfe;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.sender-name {
  font-size: 16px;
  font-weight: 700;
  color: #6c5ce7;
  margin-bottom: 8px;
}

.text-content {
  margin: 0;
  font-size: 15px;
  line-height: 1.4;
  color: var(--text-dark);
}

.media-placeholder {
  color: var(--text-muted);
  font-style: italic;
  font-size: 14px;
}

.message-footer {
  margin-top: 12px;
  font-size: 12px;
  color: var(--text-muted);
}

.error-msg {
  background: #ff7675;
  color: white;
  padding: 15px;
  border-radius: 10px;
  text-align: center;
}

.fab {
  position: fixed;
  bottom: 30px;
  right: 30px;
  width: 60px;
  height: 60px;
  border-radius: 20px;
  background-color: #e9dcf8;
  color: #6a4c93;
  border: none;
  font-size: 32px;
  display: flex;
  justify-content: center;
  align-items: center;
  box-shadow: 0 4px 15px rgba(106, 76, 147, 0.2);
  cursor: pointer;
  z-index: 100;
  transition: all 0.2s;
}

.fab:hover {
  background-color: #dec9f6;
  transform: scale(1.05);
}
</style>
