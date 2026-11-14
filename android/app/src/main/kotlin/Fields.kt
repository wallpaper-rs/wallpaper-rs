package io.github.wallpaper_rs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONObject

@Composable
fun Field(field: JSONObject, folded: Boolean, fold: () -> Unit, set: (String, Any?) -> Unit, press: () -> Unit, pick: (Boolean) -> Unit, secret: (String) -> Unit) {
    val (key, label) = field.getString("key") to field.getString("label")
    val (value, enabled) = field.opt("value") to field.optBoolean("enabled", true)
    val padded = Modifier.fillMaxWidth().padding(16.dp, 4.dp)
    when (field.getString("kind")) {
        "slider" -> Slide(field, padded, enabled) { set(key, it) }
        "bool" -> Toggle(label, value == true, enabled) { set(key, it) }
        "combo" -> {
            val options = field.optJSONArray("options").let { all -> (0 until (all?.length() ?: 0)).map { all!!.getJSONObject(it) } }
            Choices(label, options.map { it.get("value").toString() to it.getString("label") }, value.toString()) { chosen ->
                if (enabled) options.firstOrNull { it.get("value").toString() == chosen }?.let { set(key, it.get("value")) }
            }
        }
        "color" -> Tint(label, value as? String ?: "", field.optBoolean("alpha"), padded, enabled) { set(key, it) }
        "text" -> Entry(label, value as? String ?: "", padded, enabled, field.optBoolean("secret"), field.optBoolean("filled"), Typing.of(field)) {
            if (field.optBoolean("secret")) secret(it) else set(key, it)
        }
        "group" -> Row(Modifier.fillMaxWidth().clickable(onClick = fold).padding(16.dp, 8.dp), verticalAlignment = Alignment.CenterVertically) {
            Glyph(if (folded) Ph.CaretRight else Ph.CaretDown, null)
            Text(label, Modifier.padding(start = 8.dp), style = MaterialTheme.typography.titleSmall)
        }
        "label" -> Text(label, padded, style = MaterialTheme.typography.bodyMedium)
        "divider" -> Column(padded) {
            HorizontalDivider()
            if (label.isNotEmpty()) Text(label, Modifier.padding(top = 4.dp), style = MaterialTheme.typography.labelMedium)
        }
        "shortcut" -> Shortcut(field, padded, enabled) { set(key, it) }
        "button" -> QuietAction(press, Modifier.padding(16.dp, 4.dp), enabled = enabled) {
            icon(field.optString("icon"))?.let { Glyph(it, null, Modifier.padding(end = 8.dp)) }
            Text(label)
        }
        "file", "folder" -> Row(padded, verticalAlignment = Alignment.CenterVertically) {
            Column(Modifier.weight(1f)) {
                Text(label, style = MaterialTheme.typography.titleSmall)
                val shown = (value as? String)?.takeIf { it.isNotEmpty() }?.let { if (field.getString("kind") == "folder") stringResource(R.string.folder_chosen) else stringResource(R.string.file_chosen) }
                Text(shown ?: stringResource(R.string.nothing_chosen), style = MaterialTheme.typography.bodySmall)
            }
            WordAction({ pick(true) }, enabled = enabled) { Text(stringResource(R.string.choose_file)) }
            if (value is String && value.isNotEmpty()) WordAction({ pick(false) }, enabled = enabled) { Text(stringResource(R.string.remove)) }
        }
        else -> Text(stringResource(R.string.not_on_phone, label), padded, style = MaterialTheme.typography.bodySmall)
    }
    field.optString("help").takeIf { it.isNotEmpty() }?.let { Text(it, Modifier.padding(16.dp, 0.dp), style = MaterialTheme.typography.bodySmall) }
}

@Composable
fun Saved(name: String, rename: () -> Unit, export: () -> Unit, forget: () -> Unit) = Row(Modifier.fillMaxWidth().padding(horizontal = 16.dp), verticalAlignment = Alignment.CenterVertically) {
    Text(name, Modifier.weight(1f), style = MaterialTheme.typography.bodyMedium)
    IconButton(rename) { Glyph(Ph.PencilSimple, stringResource(R.string.rename_preset)) }
    IconButton(export) { Glyph(Ph.ShareNetwork, stringResource(R.string.export_preset)) }
    IconButton(forget) { Glyph(Ph.Trash, stringResource(R.string.delete_preset)) }
}

@Composable
fun Naming(name: String, close: () -> Unit, done: (String) -> Unit) {
    var typed by remember(name) { mutableStateOf(name) }
    AlertDialog(
        onDismissRequest = close,
        confirmButton = { WordAction({ done(typed) }, enabled = typed.isNotBlank()) { Text(stringResource(R.string.save_preset)) } },
        dismissButton = { WordAction(close) { Text(stringResource(R.string.cancel)) } },
        title = { Text(stringResource(R.string.preset_name)) },
        text = { OutlinedTextField(typed, { typed = it }, singleLine = true) },
    )
}

private fun icon(name: String): Ph? = when (name) {
    "appointment-soon", "x-office-calendar" -> Ph.CalendarBlank
    "dialog-information" -> Ph.Info
    "edit-find" -> Ph.MagnifyingGlass
    "emblem-favorite" -> Ph.Heart
    "go-home" -> Ph.House
    "media-playback-start" -> Ph.Play
    "media-playlist-repeat", "view-refresh" -> Ph.ArrowClockwise
    "preferences-system", "preferences-desktop-theme" -> Ph.GearSix
    "zoom-in" -> Ph.Plus
    else -> null
}
