package io.github.wallpaper_rs

import android.net.Uri
import android.provider.DocumentsContract
import android.provider.DocumentsContract.Document
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlin.concurrent.thread

@Composable
fun Adding(main: Main, sources: () -> Unit, close: () -> Unit) {
    var animated by remember { mutableStateOf(false) }
    if (animated) return Sheet(close, stringResource(R.string.animated)) {
        SheetOption(stringResource(R.string.examples), ph = Ph.Sparkle) { close(); main.examples() }
        SheetOption(stringResource(R.string.open_package), ph = Ph.Files) { close(); main.openPackage() }
    }
    Sheet(close, stringResource(R.string.choose)) {
        listOf<Triple<Int, Ph, () -> Unit>>(
            Triple(R.string.photos, Ph.Images, { main.choose() }),
            Triple(R.string.files, Ph.Files, { main.chooseFiles() }),
            Triple(R.string.folder, Ph.FolderSimple, { main.chooseFolder() }),
            Triple(R.string.animated, Ph.Sparkle, { animated = true }),
            Triple(R.string.from_internet, Ph.GlobeSimple, sources),
        ).forEach { (label, ph, pick) ->
            val sub = when (label) {
                R.string.photos -> stringResource(R.string.photos_sub)
                R.string.files -> stringResource(R.string.files_sub)
                else -> null
            }
            SheetOption(stringResource(label), ph = ph, sub = sub) {
                if (label != R.string.animated) close()
                pick()
            }
        }
    }
}

@Composable
fun Importing(main: Main) {
    val (left, total) = main.importing.intValue to main.imports.intValue
    Conversions()
    if (left <= 0) return
    val bar = Modifier.fillMaxWidth().padding(16.dp, 8.dp)
    if (total <= 0) LinearProgressIndicator(bar) else {
        LinearProgressIndicator({ (total - left).toFloat() / total }, bar)
        Note(stringResource(R.string.imported, total - left, total))
    }
}

val emptyFolder = mutableStateOf(false)

@Composable
fun EmptyFolder(main: Main) {
    if (!emptyFolder.value) return
    val close = { emptyFolder.value = false }
    Sheet(close, stringResource(R.string.folder_empty), stringResource(R.string.folder_empty_hint)) {
        Row(Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e3), Arrangement.spacedBy(Measure.e3)) {
            QuietAction(close) { Text(stringResource(R.string.cancel)) }
            Action({ close(); main.chooseFolder() }, Modifier.weight(1f).testTag("choose_other")) { Glyph(Ph.FolderSimple, null); Text(stringResource(R.string.choose_other)) }
        }
    }
}

private val COLUMNS = arrayOf(Document.COLUMN_DOCUMENT_ID, Document.COLUMN_MIME_TYPE)
private const val MAX_FILES = 5000

fun Main.importFolder(tree: Uri) {
    importing.intValue++
    thread {
        val found = runCatching { media(tree) }.getOrDefault(emptyList())
        runOnUiThread {
            importing.intValue--
            if (found.isEmpty()) emptyFolder.value = true else store(found)
        }
    }
}

private fun Main.media(tree: Uri): List<Uri> {
    val (found, pending) = mutableListOf<Uri>() to ArrayDeque(listOf(DocumentsContract.getTreeDocumentId(tree)))
    while (pending.isNotEmpty() && found.size < MAX_FILES) {
        val children = DocumentsContract.buildChildDocumentsUriUsingTree(tree, pending.removeFirst())
        contentResolver.query(children, COLUMNS, null, null, null)?.use { row ->
            while (row.moveToNext()) {
                val (id, type) = row.getString(0) to row.getString(1).orEmpty()
                when {
                    type == Document.MIME_TYPE_DIR -> pending += id
                    type.startsWith("image/") || type.startsWith("video/") -> found += DocumentsContract.buildDocumentUriUsingTree(tree, id)
                }
            }
        }
    }
    return found.take(MAX_FILES)
}
