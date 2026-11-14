package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp

private const val SEAM = 0.55f
private const val BESIDE_CHARS = 22
private val BESIDE_WIDTH = 168.dp

fun valueBeside(value: String?) = value != null && value.length <= BESIDE_CHARS

@Composable
fun Setting(
    label: String,
    modifier: Modifier = Modifier,
    sub: String? = null,
    value: String? = null,
    enabled: Boolean = true,
    onClick: (() -> Unit)? = null,
    trailing: (@Composable () -> Unit)? = null,
) {
    val t = tones
    val tap = onClick?.let { Modifier.clickable(enabled = enabled, role = Role.Button, onClick = it) } ?: Modifier
    Row(
        modifier.fillMaxWidth().then(tap).heightIn(min = Measure.row).padding(horizontal = Measure.margin, vertical = 10.dp),
        horizontalArrangement = Arrangement.spacedBy(Measure.e3),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        val beside = valueBeside(value)
        Column(Modifier.weight(1f).testTag("setting_name")) {
            Text(label, color = if (enabled) t.frost else t.mist, style = Type.row)
            sub?.let { Text(it, Modifier.padding(top = 2.dp), t.mist, style = Type.foot) }
            if (!beside) value?.let { Text(it, Modifier.padding(top = 2.dp).testTag("setting_value"), t.mist, style = Type.text) }
        }
        if (beside) value?.let { Text(it, Modifier.widthIn(max = BESIDE_WIDTH).testTag("setting_value"), t.mist, style = Type.text) }
        when {
            trailing != null -> trailing()
            onClick != null -> Glyph(Ph.CaretRight, null, tint = t.mistLow, size = Measure.iconSmall)
        }
    }
}

@Composable
fun Seam() = Spacer(Modifier.fillMaxWidth().height(Measure.line).background(tones.nightLine.copy(alpha = SEAM)))

@Composable
fun Flip(on: Boolean, modifier: Modifier = Modifier) {
    val t = tones
    val track = RoundedCornerShape(Measure.switchHeight / 2)
    Box(
        modifier.size(Measure.switchWidth, Measure.switchHeight).background(if (on) t.frost else t.nightLine, track)
            .border(2.dp, if (on) t.frost else t.mistLow, track),
        contentAlignment = if (on) Alignment.CenterEnd else Alignment.CenterStart,
    ) {
        val knob = if (on) Measure.knobOn else Measure.knobOff
        Box(Modifier.padding(horizontal = if (on) 3.dp else 5.dp).size(knob).background(if (on) t.night else t.mistLow, CircleShape))
    }
}

@Composable
fun Group(label: String, modifier: Modifier = Modifier) =
    Text(label, modifier.padding(start = Measure.margin, end = Measure.margin, top = Measure.e5, bottom = Measure.e1), tones.mist, style = Type.group)

@Composable
fun Fold(label: String, open: Boolean, modifier: Modifier = Modifier, toggle: () -> Unit) = Row(
    modifier.fillMaxWidth().clickable(role = Role.Button, onClick = toggle).heightIn(min = Measure.touch).padding(horizontal = Measure.margin),
    horizontalArrangement = Arrangement.spacedBy(Measure.e2),
    verticalAlignment = Alignment.CenterVertically,
) {
    Glyph(if (open) Ph.CaretDown else Ph.CaretRight, null, tint = tones.mist, size = Measure.iconFold)
    Text(label, color = tones.mist, style = Type.fold)
}
