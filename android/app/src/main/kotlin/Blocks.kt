package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

enum class Tone(val ph: Ph) { Amber(Ph.Warning), Coral(Ph.WarningCircle), Sprout(Ph.CheckCircle) }

private val INDENT = 31.dp

@Composable
fun Tone.color() = when (this) {
    Tone.Amber -> tones.amber
    Tone.Coral -> tones.coral
    Tone.Sprout -> tones.sprout
}

@Composable
fun StateLine(text: String, tone: Tone, modifier: Modifier = Modifier, single: Boolean = false) =
    Row(modifier, horizontalArrangement = Arrangement.spacedBy(6.dp)) {
        val ink = tone.color()
        Glyph(tone.ph, null, Modifier.padding(top = 1.dp), tint = ink, size = 16.dp)
        Text(text, color = ink, style = Type.foot.copy(fontWeight = Type.row.fontWeight), maxLines = if (single) 1 else Int.MAX_VALUE, overflow = TextOverflow.Ellipsis)
    }

@Composable
fun Reason(text: String, modifier: Modifier = Modifier) =
    Row(modifier.padding(top = Measure.e2), horizontalArrangement = Arrangement.spacedBy(Measure.e2)) {
        Glyph(Ph.Info, null, Modifier.padding(top = 1.dp), tint = tones.mist, size = 16.dp)
        Text(text, color = tones.mist, style = Type.foot)
    }

@Composable
fun Blank(title: String, text: String?, modifier: Modifier = Modifier, actions: @Composable ColumnScope.() -> Unit = {}) =
    Column(modifier.padding(horizontal = Measure.margin)) {
        Text(title, style = Type.empty)
        text?.let { Text(it, Modifier.padding(top = Measure.e2), tones.mist, style = Type.text) }
        Column(Modifier.padding(top = Measure.e5).fillMaxWidth(), Arrangement.spacedBy(Measure.e3), content = actions)
    }

@Composable
fun Problem(title: String, text: String?, modifier: Modifier = Modifier, details: String? = null, actions: @Composable ColumnScope.() -> Unit = {}) =
    Column(modifier.padding(horizontal = Measure.margin, vertical = Measure.e4)) {
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            Glyph(Ph.WarningCircle, null, Modifier.padding(top = 1.dp), tint = tones.coral, size = 21.dp)
            Text(title, style = Type.row.copy(fontFamily = RedHat, fontWeight = Type.button.fontWeight))
        }
        Column(Modifier.padding(start = INDENT)) {
            text?.let { Text(it, Modifier.padding(top = 6.dp), tones.mist, style = Type.text) }
            Column(Modifier.padding(top = Measure.e4), Arrangement.spacedBy(Measure.e3), content = actions)
            details?.let { more ->
                var open by rememberSaveable { mutableStateOf(false) }
                Fold(stringResource(R.string.details), open, Modifier.padding(start = 0.dp)) { open = !open }
                if (open) Text(more, color = tones.mist, style = Type.foot)
            }
        }
    }

@Composable
fun Progress(words: String, value: Float?, modifier: Modifier = Modifier, stop: (() -> Unit)? = null) =
    Column(modifier.padding(horizontal = Measure.margin, vertical = Measure.e2)) {
        Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(Measure.e3)) {
            Text(words, Modifier.weight(1f), style = Type.foot)
            stop?.let { WordAction(it) { Text(stringResource(R.string.stop)) } }
        }
        val t = tones
        Box(Modifier.padding(top = 6.dp).fillMaxWidth().height(Measure.rail).clip(RoundedCornerShape(2.dp)).background(t.nightLine)) {
            Box(Modifier.fillMaxWidth(value?.coerceIn(0f, 1f) ?: 1f).height(Measure.rail).background(if (value == null) t.mistLow else t.frost))
        }
    }

@Composable
fun Tell(text: String, modifier: Modifier = Modifier, ph: Ph? = null, action: (@Composable () -> Unit)? = null) {
    val t = tones
    val shape = RoundedCornerShape(Measure.rNotice)
    Row(
        modifier.fillMaxWidth().padding(horizontal = Measure.e3).heightIn(min = Measure.option)
            .background(t.nightHigh, shape).border(Measure.line, t.nightLine, shape).padding(start = Measure.e4, end = 6.dp, top = Measure.e2, bottom = Measure.e2),
        horizontalArrangement = Arrangement.spacedBy(Measure.e3),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        ph?.let { Glyph(it, null, size = Measure.iconSquare) }
        Text(text, Modifier.weight(1f), style = Type.text)
        action?.invoke()
    }
}
