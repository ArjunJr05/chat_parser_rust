<script setup>
import { ref, onMounted } from 'vue';
import { initWasm, parse_chat_wasm, decodeWhatsAppExport } from './wasm-init';

const status = ref('Initializing WASM...');
const result = ref('');

onMounted(async () => {
  try {
    await initWasm();
    status.value = 'WASM Ready!';
  } catch (error) {
    status.value = 'WASM Initialization Failed: ' + error.message;
    console.error(error);
  }
});

const handleFileUpload = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  status.value = 'Processing file...';
  try {
    const arrayBuffer = await file.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);
    
    // 1. Parse ZIP with WASM (returns Protobuf bytes)
    const protoBytes = parse_chat_wasm(bytes);
    
    // 2. Deserialize Protobuf to JS Object
    const decodedObject = decodeWhatsAppExport(protoBytes);
    
    // 3. Display the result
    result.value = JSON.stringify(decodedObject, null, 2);

    status.value = 'WASM Ready!';
    console.log('Deserialized Output:', decodedObject);
  } catch (error) {
    result.value = 'Error processing file: ' + error.message;
    status.value = 'Processing Error';
    console.error(error);
  }
};
</script>

<template>
  <div class="container">
    <header>
      <h1>Vue.js + Rust WASM Integration</h1>
      <p class="status" :class="{ 'ready': status === 'WASM Ready!', 'error': status.startsWith('WASM Initialization Failed') || status === 'Processing Error' }">
        {{ status }}
      </p>
    </header>

    <main>
      <div class="card">
        <h2>Input ZIP File</h2>
        <p>Select a <code>.zip</code> file to process with Rust WASM.</p>
        
        <div class="upload-area">
          <input 
            type="file" 
            accept=".zip" 
            @change="handleFileUpload" 
            :disabled="status !== 'WASM Ready!'"
            id="zip-input"
          />
          <label for="zip-input" class="file-label">
            {{ status === 'Processing file...' ? '⚙️ Processing...' : '📁 Choose ZIP File' }}
          </label>
        </div>

        <p v-if="result" class="output-label">WASM Output:</p>
        <pre v-if="result" class="result">{{ result }}</pre>
      </div>

      <div class="info">
        <h3>How it works:</h3>
        <ul>
          <li>The file is read using the <code>FileReader</code> API (via <code>file.arrayBuffer()</code>).</li>
          <li>It is converted into a <code>Uint8Array</code>.</li>
          <li>That array is passed directly to your Rust <code>parse_chat_wasm</code> function.</li>
        </ul>
      </div>
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;

  color-scheme: light dark;
  color: rgba(255, 255, 255, 0.87);
  background-color: #242424;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
}

#app {
  max-width: 1280px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

header h1 {
  background: linear-gradient(135deg, #42b883, #35495e);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  font-size: 2.5rem;
}

.status {
  font-weight: bold;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  display: inline-block;
  background: #333;
}

.status.ready {
  color: #42b883;
  border: 1px solid #42b88333;
}

.status.error {
  color: #ff4757;
  border: 1px solid #ff475733;
}

.card {
  background: #1a1a1a;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0,0,0,0.3);
  margin-bottom: 1rem;
}

.upload-area {
  margin: 2rem 0;
}

#zip-input {
  display: none;
}

.file-label {
  background-color: #42b883;
  color: white;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-block;
  box-shadow: 0 4px 6px rgba(66, 184, 131, 0.2);
}

.file-label:hover {
  background-color: #3aa876;
  transform: translateY(-2px);
  box-shadow: 0 6px 12px rgba(66, 184, 131, 0.3);
}

.file-label:active {
  transform: translateY(0);
}

button {
  background-color: #42b883;
  color: white;
  border: none;
  padding: 0.8rem 1.5rem;
  font-size: 1rem;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

button:hover:not(:disabled) {
  background-color: #3aa876;
  transform: translateY(-2px);
}

button:disabled {
  background-color: #555;
  cursor: not-allowed;
  opacity: 0.7;
}

.output-label {
  text-align: left;
  font-weight: bold;
  color: #42b883;
  margin-top: 2rem;
  font-size: 0.9rem;
  letter-spacing: 0.05rem;
  text-transform: uppercase;
}

.result {
  margin-top: 0.5rem;
  padding: 1.5rem;
  background: #0d0d0d;
  border-radius: 8px;
  font-family: 'Fira Code', 'Courier New', monospace;
  text-align: left;
  overflow-x: auto;
  border: 1px solid #333;
  color: #e0e0e0;
  font-size: 0.9rem;
  line-height: 1.4;
  white-space: pre-wrap;
  word-break: break-all;
}

.info {
  text-align: left;
  max-width: 600px;
  margin: 0 auto;
}

code {
  background: #333;
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
}
</style>
