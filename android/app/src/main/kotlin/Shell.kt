package io.github.wallpaper_rs

import android.graphics.BitmapFactory
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.statusBars
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import org.json.JSONObject

enum class Veil { Shelf, Title, Strip, Panel }

private val TOP_BAND = 104.dp
private val TITLE_BLOCK = 330.dp
private val TITLE_FADE = 440.dp
private val SHELF_SOLID = 452.dp
private val SHELF_HALF = 500.dp
private val SHELF_CLEAR = 566.dp
private val STRIP = 260.dp
private val STRIP_TOP = 96.dp
private const val FADE_STEPS = 11

@Composable
fun Shell(main: Main, veil: Veil, bar: (@Composable () -> Unit)?, content: @Composable () -> Unit) =
    Box(Modifier.fillMaxSize().background(tones.night).testTag("shell")) {
        if (veil != Veil.Panel) Backdrop(main, veil)
        Column(Modifier.fillMaxSize()) {
            val edge = if (bar == null) WindowInsets.navigationBars else WindowInsets(0.dp)
            Box(Modifier.weight(1f).fillMaxWidth().windowInsetsPadding(WindowInsets.statusBars).windowInsetsPadding(edge)) { content() }
            bar?.invoke()
        }
    }

@Composable
private fun Backdrop(main: Main, veil: Veil) {
    val poster by loaded(null, main.revision.intValue) {
        val uid = (ask("current") as? JSONObject)?.takeUnless { it.isNull("image") }?.getString("image")
        uid?.let { (ask("thumb", "uid" to it) as? String) }?.let(BitmapFactory::decodeFile)?.asImageBitmap()
    }
    val picture = poster ?: return
    val t = tones
    Image(picture, null, Modifier.fillMaxSize().testTag("poster"), Alignment.TopCenter, ContentScale.Crop)
    Box(Modifier.fillMaxSize().drawBehind { veils(veil, t) })
}

private fun DrawScope.veils(veil: Veil, t: Tones) = when (veil) {
    Veil.Shelf -> drawRect(Brush.verticalGradient(listOf(t.veiled(0.9f), t.veiled(0.95f))))
    Veil.Title -> title(t)
    Veil.Strip -> strip(t)
    Veil.Panel -> drawRect(t.night)
}

private fun DrawScope.down(t: Tones, vararg stops: Pair<Dp, Float>) {
    val tall = size.height
    val colors = stops.map { (at, alpha) -> (at.toPx() / tall).coerceIn(0f, 1f) to t.veiled(alpha) }
    drawRect(Brush.verticalGradient(*colors.toTypedArray()))
}

private fun DrawScope.across(t: Tones, top: Float, height: Float, strength: Float, vararg stops: Pair<Float, Float>) = drawRect(
    Brush.horizontalGradient(*stops.map { (at, alpha) -> at to t.veiled(alpha * strength) }.toTypedArray()),
    Offset(0f, top),
    Size(size.width, height),
)

private fun DrawScope.title(t: Tones) {
    val v = t.veil
    down(t, 0.dp to v * 0.5f, TOP_BAND to 0f)
    val bottom = size.height
    drawRect(
        Brush.verticalGradient(
            0f to t.veiled(0f),
            ((bottom - SHELF_CLEAR.toPx()) / bottom).coerceIn(0f, 1f) to t.veiled(0f),
            ((bottom - SHELF_HALF.toPx()) / bottom).coerceIn(0f, 1f) to t.veiled(0.6f),
            ((bottom - SHELF_SOLID.toPx()) / bottom).coerceIn(0f, 1f) to t.veiled(0.95f),
            1f to t.veiled(0.97f),
        ),
    )
    val left = arrayOf(0f to v * 0.84f, 0.56f to v * 0.76f, 0.73f to 0f)
    across(t, 0f, TITLE_BLOCK.toPx(), 1f, *left)
    val slice = (TITLE_FADE - TITLE_BLOCK).toPx() / FADE_STEPS
    repeat(FADE_STEPS) { step ->
        across(t, TITLE_BLOCK.toPx() + step * slice, slice, 1f - (step + 1f) / (FADE_STEPS + 1), *left)
    }
}

private fun DrawScope.strip(t: Tones) {
    val v = t.veil
    val high = STRIP.toPx()
    drawRect(
        Brush.verticalGradient(0f to t.veiled(0f), 0.52f to t.veiled(0f), 0.8f to t.veiled(0.78f), 1f to t.night, startY = 0f, endY = high),
        size = Size(size.width, high),
    )
    across(t, 0f, high, v, 0f to 0.7f, 0.55f to 0.6f, 0.74f to 0f)
    down(t, 0.dp to v * 0.5f, STRIP_TOP to 0f)
    drawRect(t.night, Offset(0f, high), Size(size.width, size.height - high))
}
