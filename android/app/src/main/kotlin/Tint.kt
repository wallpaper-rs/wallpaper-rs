package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

object Hex {
    fun parse(text: String, alpha: Boolean): List<Int>? {
        val digits = text.trim().removePrefix("#")
        if (digits.length != 6 && !(alpha && digits.length == 8)) return null
        val channels = digits.chunked(2).map { it.toIntOrNull(16) ?: return null }
        return if (channels.size == 3) channels + 255 else channels
    }

    fun format(rgba: List<Int>, alpha: Boolean): String {
        val shown = if (alpha) rgba else rgba.take(3)
        return "#" + shown.joinToString("") { "%02x".format(it.coerceIn(0, 255)) }
    }
}

private fun color(rgba: List<Int>) = Color(rgba[0], rgba[1], rgba[2], rgba[3])

@Composable
internal fun Tint(label: String, value: String, alpha: Boolean, modifier: Modifier, enabled: Boolean, keep: (String) -> Unit) {
    var open by remember { mutableStateOf(false) }
    val rgba = Hex.parse(value, alpha) ?: listOf(0, 0, 0, 255)
    Row(modifier.clickable(enabled) { open = true }, verticalAlignment = Alignment.CenterVertically) {
        Box(Modifier.size(32.dp).background(color(rgba), CircleShape).border(1.dp, MaterialTheme.colorScheme.outline, CircleShape))
        Column(Modifier.padding(start = 12.dp)) {
            Text(label, style = MaterialTheme.typography.titleSmall)
            Text(Hex.format(rgba, alpha), style = MaterialTheme.typography.bodySmall)
        }
    }
    if (open) Mixer(label, rgba, alpha, { open = false }) {
        open = false
        keep(it)
    }
}

@Composable
private fun Mixer(label: String, start: List<Int>, alpha: Boolean, close: () -> Unit, keep: (String) -> Unit) {
    var rgba by remember { mutableStateOf(start) }
    var typed by remember { mutableStateOf(Hex.format(start, alpha)) }
    val channels = listOf(R.string.red, R.string.green, R.string.blue, R.string.opacity).take(if (alpha) 4 else 3)
    AlertDialog(
        onDismissRequest = close,
        confirmButton = { TextButton({ keep(Hex.format(rgba, alpha)) }) { Text(stringResource(R.string.choose_color)) } },
        dismissButton = { TextButton(close) { Text(stringResource(R.string.cancel)) } },
        title = { Text(label) },
        text = {
            Column {
                Box(Modifier.fillMaxWidth().height(40.dp).background(color(rgba), RoundedCornerShape(8.dp)))
                channels.forEachIndexed { at, name ->
                    Text(stringResource(name, rgba[at]), Modifier.padding(top = 8.dp), style = MaterialTheme.typography.bodySmall)
                    Slider(rgba[at].toFloat(), { level ->
                        rgba = rgba.toMutableList().also { it[at] = level.toInt() }
                        typed = Hex.format(rgba, alpha)
                    }, valueRange = 0f..255f)
                }
                OutlinedTextField(typed, { text ->
                    typed = text
                    Hex.parse(text, alpha)?.let { rgba = it }
                }, singleLine = true, isError = Hex.parse(typed, alpha) == null, label = { Text(stringResource(R.string.hex_color)) })
            }
        },
    )
}
