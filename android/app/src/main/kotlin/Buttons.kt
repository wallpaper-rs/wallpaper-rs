package io.github.wallpaper_rs

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.LocalTextStyle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight

enum class Kind { Main, Quiet, Word, Remove }

private const val WORD_PRESSED = 0.12f
private const val BOXED = 0.78f
private const val REMOVE_PRESSED = 0.8f

@Composable
fun Action(onClick: () -> Unit, modifier: Modifier = Modifier, enabled: Boolean = true, kind: Kind = Kind.Main, content: @Composable RowScope.() -> Unit) {
    val press = remember { MutableInteractionSource() }
    val pressed by press.collectIsPressedAsState()
    val t = tones
    val (fill, ink) = when (kind) {
        Kind.Main -> (if (pressed) t.moonDeep else t.moon) to t.night
        Kind.Quiet -> (if (pressed) t.nightLine else t.nightHigh) to t.frost
        Kind.Word -> (if (pressed) t.frost.copy(alpha = WORD_PRESSED) else Color.Transparent) to t.frost
        Kind.Remove -> (if (pressed) t.coral.copy(alpha = REMOVE_PRESSED).compositeOver(t.night) else t.coral) to t.night
    }
    val edge = when {
        !enabled -> BorderStroke(Measure.line, t.nightLine)
        kind == Kind.Quiet -> BorderStroke(Measure.line, t.nightLine)
        else -> null
    }
    val wide = if (kind == Kind.Word) PaddingValues(horizontal = Measure.e3) else PaddingValues(start = Measure.buttonStart, end = Measure.e5)
    Button(
        onClick = onClick,
        modifier = modifier.defaultMinSize(minHeight = Measure.touch),
        enabled = enabled,
        shape = RoundedCornerShape(Measure.rControl),
        colors = ButtonDefaults.buttonColors(fill, ink, t.nightHigh, t.mistLow),
        border = edge,
        contentPadding = wide,
        interactionSource = press,
    ) {
        val weight = if (kind == Kind.Quiet) FontWeight.SemiBold else FontWeight.Bold
        CompositionLocalProvider(LocalTextStyle provides Type.button.copy(fontWeight = weight)) { content() }
    }
}

@Composable
fun QuietAction(onClick: () -> Unit, modifier: Modifier = Modifier, enabled: Boolean = true, content: @Composable RowScope.() -> Unit) =
    Action(onClick, modifier, enabled, Kind.Quiet, content)

@Composable
fun WordAction(onClick: () -> Unit, modifier: Modifier = Modifier, enabled: Boolean = true, content: @Composable RowScope.() -> Unit) =
    Action(onClick, modifier, enabled, Kind.Word, content)

@Composable
fun RemoveAction(onClick: () -> Unit, modifier: Modifier = Modifier, enabled: Boolean = true, content: @Composable RowScope.() -> Unit) =
    Action(onClick, modifier, enabled, Kind.Remove, content)

@Composable
fun Square(onClick: () -> Unit, ph: Ph, name: String, modifier: Modifier = Modifier, boxed: Boolean = true, lit: Boolean = false) {
    val t = tones
    val shape = RoundedCornerShape(Measure.rControl)
    val edge = if (boxed) Modifier.background(t.veiled(BOXED), shape).border(Measure.line, t.nightLine, shape) else Modifier
    Box(
        modifier.size(Measure.touch).then(edge).clip(shape).clickable(role = Role.Button, onClick = onClick),
        contentAlignment = Alignment.Center,
    ) { Glyph(ph, name, lit = lit, tint = t.frost, size = Measure.iconSquare) }
}
