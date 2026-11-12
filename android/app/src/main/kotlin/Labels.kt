package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Clear
import androidx.compose.material.icons.filled.Star
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.InputChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

private const val STARS = 5

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Labels(main: Main, item: Item) = Column(Modifier.padding(horizontal = 16.dp), Arrangement.spacedBy(4.dp)) {
    var tagging by remember { mutableStateOf(false) }
    var renaming by remember { mutableStateOf(false) }
    val uids = listOf(item.uid)
    Row(verticalAlignment = Alignment.CenterVertically) {
        (1..STARS).forEach { star ->
            val lit = star <= item.rating
            IconButton({ main.act { ask("rate", "uids" to uids, "rating" to if (item.rating == star) 0 else star) } }) {
                val tint = if (lit) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.outlineVariant
                Icon(Icons.Default.Star, stringResource(R.string.rate_stars, star), tint = tint)
            }
        }
    }
    FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        item.tags.forEach { tag ->
            InputChip(
                false, { main.act { ask("retag", "uids" to uids, "add" to emptyList<String>(), "remove" to listOf(tag)) } }, { Text(tag) },
                trailingIcon = { Icon(Icons.Default.Clear, stringResource(R.string.tag_remove, tag)) },
            )
        }
    }
    Row {
        TextButton({ tagging = true }) { Text(stringResource(R.string.tag_add)) }
        TextButton({ renaming = true }) { Text(stringResource(R.string.rename)) }
    }
    if (tagging) Asking(R.string.tag_add, R.string.tag_name, "", { tagging = false }) { text ->
        main.act { ask("retag", "uids" to uids, "add" to text.split(',').map(String::trim).filter(String::isNotEmpty), "remove" to emptyList<String>()) }
    }
    if (renaming) Asking(R.string.rename, R.string.title_label, item.title, { renaming = false }) { text ->
        main.act { ask("rename", "uid" to item.uid, "title" to text) }
    }
}

@Composable
private fun Asking(title: Int, label: Int, start: String, close: () -> Unit, done: (String) -> Unit) {
    var text by remember { mutableStateOf(start) }
    AlertDialog(
        onDismissRequest = close,
        title = { Text(stringResource(title)) },
        text = { OutlinedTextField(text, { text = it }, Modifier.fillMaxWidth(), singleLine = true, label = { Text(stringResource(label)) }) },
        confirmButton = { TextButton({ close(); done(text) }, enabled = text.isNotBlank()) { Text(stringResource(R.string.save)) } },
        dismissButton = { TextButton(close) { Text(stringResource(R.string.cancel)) } },
    )
}
