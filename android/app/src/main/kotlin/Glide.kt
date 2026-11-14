package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import java.util.Locale

@Composable
fun Glide(label: String, saved: Float, range: ClosedFloatingPointRange<Float>, modifier: Modifier = Modifier, save: (Float) -> Unit) {
    var value by remember(saved) { mutableFloatStateOf(saved.coerceIn(range)) }
    Rail(label, shown(value, range.endInclusive - range.start), value, range, modifier = modifier, change = { value = it }, save = save)
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun Rail(
    label: String, value: String, at: Float, range: ClosedFloatingPointRange<Float>, steps: Int = 0, enabled: Boolean = true,
    modifier: Modifier = Modifier, change: (Float) -> Unit, save: (Float) -> Unit,
) {
    val t = tones
    val span = range.endInclusive - range.start
    val nudge = { by: Float ->
        val moved = (at + by * if (steps > 0) span / (steps + 1) else span / GLIDE_NUDGES).coerceIn(range)
        change(moved)
        save(moved)
    }
    val (lit, rail) = if (enabled) t.frost to t.nightLine else t.mistLow to t.nightLine
    Column(modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e1).testTag("glide")) {
        Row(Modifier.fillMaxWidth(), Arrangement.SpaceBetween) {
            Text(label, Modifier.weight(1f), color = if (enabled) Color.Unspecified else t.mistLow, style = Type.row)
            Text(value, color = lit, style = Type.row.copy(fontWeight = FontWeight.Bold))
        }
        Row(Modifier.fillMaxWidth(), Arrangement.spacedBy(Measure.e2), Alignment.CenterVertically) {
            Square({ if (enabled) nudge(-1f) }, Ph.Minus, stringResource(R.string.glide_less), boxed = false)
            Slider(
                at, change, Modifier.weight(1f), enabled = enabled, valueRange = range, steps = steps, onValueChangeFinished = { save(at) },
                thumb = { Box(Modifier.size(GLIDE_THUMB).background(lit, CircleShape)) },
                track = {
                    val share = if (span > 0f) (at - range.start) / span else 0f
                    Box(Modifier.fillMaxWidth().height(GLIDE_RAIL).background(rail, CircleShape)) {
                        Box(Modifier.fillMaxWidth(share).height(GLIDE_RAIL).background(lit, CircleShape))
                    }
                },
            )
            Square({ if (enabled) nudge(1f) }, Ph.Plus, stringResource(R.string.glide_more), boxed = false)
        }
    }
}

private fun shown(value: Float, span: Float) = when {
    span > WHOLE_SPAN -> Math.round(value).toString()
    span < FINE_SPAN -> String.format(Locale.ROOT, "%.3f", value)
    else -> String.format(Locale.ROOT, "%.2f", value)
}

private const val GLIDE_NUDGES = 20f
private const val WHOLE_SPAN = 8f
private const val FINE_SPAN = 0.1f
private val GLIDE_THUMB = 22.dp
private val GLIDE_RAIL = 4.dp
