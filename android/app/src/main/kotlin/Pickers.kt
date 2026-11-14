package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role

data class Word(val value: String, val word: String, val sub: String? = null)

@Composable
fun Switch(label: String, checked: Boolean, sub: String? = null, change: (Boolean) -> Unit) = Setting(
    label,
    Modifier.toggleable(value = checked, role = Role.Switch, onValueChange = change),
    sub = sub,
    trailing = { Flip(checked) },
)

@Composable
fun Pick(label: String, words: List<Word>, chosen: String, inSub: Boolean = false, more: List<Word> = emptyList(), set: (String) -> Unit) {
    var open by remember { mutableStateOf(false) }
    val shown = (words + more).firstOrNull { it.value == chosen }?.word ?: chosen
    Setting(label, Modifier.testTag("pick"), sub = shown.takeIf { inSub }, value = shown.takeUnless { inSub }, onClick = { open = true })
    if (open) {
        val close = { open = false }
        Sheet(close, label) {
            words.forEach { SheetOption(it.word, sub = it.sub, chosen = it.value == chosen) { set(it.value).also { close() } } }
            if (more.isNotEmpty()) {
                var unfolded by rememberSaveable { mutableStateOf(words.none { it.value == chosen }) }
                Fold(stringResource(R.string.more_options), unfolded) { unfolded = !unfolded }
                if (unfolded) more.forEach { SheetOption(it.word, sub = it.sub, chosen = it.value == chosen) { set(it.value).also { close() } } }
            }
        }
    }
}

const val PAUSE_STEP = 5

fun pauseLimit(value: String) = value.removePrefix("below:").toIntOrNull()?.takeIf { value.startsWith("below:") }

@Composable
fun PausePick(label: String, chosen: String, set: (String) -> Unit) {
    var open by remember { mutableStateOf(false) }
    val limit = pauseLimit(chosen)
    val shown = when {
        limit != null -> stringResource(R.string.below_short, limit)
        chosen == "always" -> stringResource(R.string.pause_always)
        else -> stringResource(R.string.never)
    }
    Setting(label, Modifier.testTag("pause"), value = shown, onClick = { open = true })
    if (!open) return
    val close = { open = false }
    Sheet(close, stringResource(R.string.on_battery)) {
        SheetOption(stringResource(R.string.never), chosen = chosen == "never" || chosen.isEmpty()) { set("never").also { close() } }
        SheetOption(stringResource(R.string.pause_always), chosen = chosen == "always") { set("always").also { close() } }
        val level = limit ?: PAUSE_DEFAULT
        SheetOption(stringResource(R.string.below_percent, level), chosen = limit != null) { set("below:$level") }
        Row(
            Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e1),
            Arrangement.spacedBy(Measure.e3),
            Alignment.CenterVertically,
        ) {
            Square({ set("below:${(level - PAUSE_STEP).coerceAtLeast(PAUSE_STEP)}") }, Ph.Minus, stringResource(R.string.less))
            Text("$level %", style = Type.digits)
            Square({ set("below:${(level + PAUSE_STEP).coerceAtMost(PAUSE_TOP)}") }, Ph.Plus, stringResource(R.string.more))
        }
    }
}

private const val PAUSE_DEFAULT = 20
private const val PAUSE_TOP = 95
