package com.example.imported_rust

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import android.webkit.MimeTypeMap
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import whatsapp.WhatsAppExport
import whatsapp.WhatsAppMessage
import java.io.File
import java.io.FileOutputStream
import java.io.InputStream
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

class MainActivity : ComponentActivity() {

    private lateinit var whatsAppConnector: WhatsAppAndroidConnector
    private var exportData by mutableStateOf<WhatsAppExport?>(null)
    private var statusMessage by mutableStateOf("Select or share a WhatsApp export zip to see the data.")

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        try {
            whatsAppConnector = WhatsAppAndroidConnector()
        } catch (e: Exception) {
            Log.e("MainActivity", "Error creating WhatsAppAndroidConnector", e)
            statusMessage = "Error: Could not initialize native engine."
        }

        setContent {
            MaterialTheme {
                MainScreen(
                    exportData = exportData,
                    statusMessage = statusMessage,
                    onFileSelected = { uri -> processUri(uri) }
                )
            }
        }

        if (::whatsAppConnector.isInitialized) {
            handleIntent(intent)
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
        if (::whatsAppConnector.isInitialized) {
            handleIntent(intent)
        }
    }

    private fun handleIntent(intent: Intent?) {
        intent?.let {
            if (it.action == Intent.ACTION_SEND || it.action == Intent.ACTION_SEND_MULTIPLE) {
                if (it.action == Intent.ACTION_SEND) {
                    val uri = it.getParcelableExtra<Uri>(Intent.EXTRA_STREAM)
                    if (uri != null) processUri(uri)
                } else if (it.action == Intent.ACTION_SEND_MULTIPLE) {
                    val uris = it.getParcelableArrayListExtra<Uri>(Intent.EXTRA_STREAM)
                    uris?.firstOrNull()?.let { uri -> processUri(uri) }
                }
            }
        }
    }

    private fun processUri(uri: Uri) {
        statusMessage = "Processing file..."
        val tempFile = copyUriToTempFile(uri)
        if (tempFile != null) {
            Log.d("MainActivity", "Processing file: ${tempFile.absolutePath}")
            val protoBytes = whatsAppConnector.parseChatAndGetProtoBytes(tempFile.absolutePath)
            if (protoBytes != null) {
                try {
                    val export = WhatsAppExport.parseFrom(protoBytes)
                    exportData = export
                    statusMessage = "Successfully parsed: ${export.chatName}"
                } catch (e: Exception) {
                    Log.e("MainActivity", "Failed to deserialize Protobuf: ${e.message}")
                    statusMessage = "Failed to deserialize data."
                }
            } else {
                Log.e("MainActivity", "Rust core returned null.")
                statusMessage = "Rust engine failed to parse."
            }
            tempFile.delete()
        } else {
            Log.e("MainActivity", "Failed to copy URI to temp file: $uri")
            statusMessage = "Failed to read file."
        }
    }

    private fun copyUriToTempFile(uri: Uri): File? {
        return try {
            val inputStream: InputStream? = contentResolver.openInputStream(uri)
            val mimeType = contentResolver.getType(uri)
            val extension = MimeTypeMap.getSingleton().getExtensionFromMimeType(mimeType) ?: "zip"
            val tempFile = File(cacheDir, "${System.currentTimeMillis()}_shared_file.$extension")
            val outputStream = FileOutputStream(tempFile)
            inputStream?.copyTo(outputStream)
            inputStream?.close()
            outputStream.close()
            tempFile
        } catch (e: Exception) {
            Log.e("MainActivity", "Error copying URI to temp file: ${e.message}", e)
            null
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
    exportData: WhatsAppExport?,
    statusMessage: String,
    onFileSelected: (Uri) -> Unit
) {
    val launcher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.GetContent()
    ) { uri: Uri? ->
        uri?.let { onFileSelected(it) }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("WhatsApp Parser") },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.primary,
                )
            )
        },
        floatingActionButton = {
            FloatingActionButton(onClick = { launcher.launch("application/zip") }) {
                Icon(Icons.Default.Add, contentDescription = "Pick Zip File")
            }
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize()
                .padding(16.dp)
        ) {
            Text(
                text = statusMessage,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.secondary
            )

            HorizontalDivider(modifier = Modifier.padding(vertical = 12.dp))

            if (exportData != null) {
                Text(
                    text = "Chat: ${exportData.chatName}",
                    style = MaterialTheme.typography.titleLarge,
                    modifier = Modifier.padding(bottom = 8.dp)
                )

                LazyColumn(modifier = Modifier.fillMaxSize()) {
                    items(exportData.messagesList) { message ->
                        MessageItem(message)
                        Spacer(modifier = Modifier.height(8.dp))
                    }
                }
            } else {
                Spacer(modifier = Modifier.height(32.dp))
                Button(
                    onClick = { launcher.launch("application/zip") },
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Text("Select WhatsApp Export Zip")
                }
            }
        }
    }
}

@Composable
fun MessageItem(message: WhatsAppMessage) {
    val dateFormat = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())

    Card(
        modifier = Modifier.fillMaxWidth(),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            when {
                message.hasText() -> {
                    val msg = message.text
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "TEXT MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(text = msg.text, style = MaterialTheme.typography.bodyLarge)
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(text = "Sent at: $dateStr • Type: ${msg.base.type}", style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.secondary)
                }
                message.hasImage() -> {
                    val msg = message.image
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "IMAGE MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(text = "Name: ${msg.name}", style = MaterialTheme.typography.bodyMedium)
                    Text(
                        text = "Dimensions: ${msg.width} x ${msg.height}\n" +
                                "Size: ${msg.size / 1024} KB\n" +
                                "Extension: ${msg.extension}\n" +
                                "Sent at: $dateStr • Type: ${msg.base.type}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.secondary
                    )
                }
                message.hasVideo() -> {
                    val msg = message.video
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "VIDEO MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(text = "Name: ${msg.name}", style = MaterialTheme.typography.bodyMedium)
                    Text(
                        text = "Duration: ${msg.duration}\n" +
                                "Size: ${msg.size / 1024} KB\n" +
                                "Extension: ${msg.extension}\n" +
                                "Sent at: $dateStr • Type: ${msg.base.type}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.secondary
                    )
                }
                message.hasAudio() -> {
                    val msg = message.audio
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "AUDIO MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(text = "Name: ${msg.name}", style = MaterialTheme.typography.bodyMedium)
                    Text(
                        text = "Duration: ${msg.duration}\n" +
                                "Size: ${msg.size / 1024} KB\n" +
                                "Sent at: $dateStr • Type: ${msg.base.type}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.secondary
                    )
                }
                message.hasDocument() -> {
                    val msg = message.document
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "DOCUMENT MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(text = "Name: ${msg.name}.${msg.extension}", style = MaterialTheme.typography.bodyMedium)
                    Text(
                        text = "Size: ${msg.size / 1024} KB\n" +
                                "Sent at: $dateStr • Type: ${msg.base.type}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.secondary
                    )
                }
                message.hasSticker() -> {
                    val msg = message.sticker
                    val dateStr = dateFormat.format(Date(msg.base.timestamp.seconds * 1000))
                    Text(text = "STICKER MESSAGE", style = MaterialTheme.typography.labelSmall, color = MaterialTheme.colorScheme.secondary)
                    Text(text = msg.base.sender, fontWeight = FontWeight.Bold, color = MaterialTheme.colorScheme.primary)
                    Text(
                        text = "Sent at: $dateStr • Type: ${msg.base.type}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.secondary
                    )
                }
            }
        }
    }
}
