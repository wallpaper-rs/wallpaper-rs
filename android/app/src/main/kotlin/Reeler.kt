package io.github.wallpaper_rs

import android.graphics.BitmapFactory
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.Image
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.RangeSlider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.produceState
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

fun clock(seconds: Float) = "%d:%02d.%d".format(seconds.toInt() / 60, seconds.toInt() % 60, (seconds * 10).toInt() % 10)

private fun JSONObject.number(key: String) = optJSONArray("fields")?.let { all ->
    (0 until all.length()).map(all::getJSONObject).firstOrNull { it.optString("key") == key }?.optDouble("value")?.toFloat()
} ?: 0f

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun Trim(length: Float, start: Float, end: Float, done: (Float, Float) -> Unit) {
    var range by remember(start, end, length) { mutableStateOf(start..(if (end <= 0f) length else end).coerceAtMost(length)) }
    Column(Modifier.fillMaxWidth().padding(16.dp, 4.dp)) {
        Text(stringResource(R.string.video_trim), style = MaterialTheme.typography.titleSmall)
        RangeSlider(range, { range = it }, valueRange = 0f..length, onValueChangeFinished = {
            done(range.start, if (range.endInclusive >= length - 0.05f) 0f else range.endInclusive)
        })
        Row {
            Text(clock(range.start), style = MaterialTheme.typography.bodySmall)
            Text(" – " + clock(range.endInclusive), style = MaterialTheme.typography.bodySmall)
        }
    }
}

@Composable
fun Framing(uid: String, zoom: Float, x: Float, y: Float, done: (Float, Float) -> Unit) {
    val poster by produceState<android.graphics.Bitmap?>(null, uid) {
        value = withContext(Dispatchers.IO) { (ask("thumb", "uid" to uid) as? String)?.let(BitmapFactory::decodeFile) }
    }
    val picture = poster ?: return
    val px = remember(x, y) { mutableFloatStateOf(x) }
    val py = remember(x, y) { mutableFloatStateOf(y) }
    val scale = zoom.coerceAtLeast(1f)
    val edge = MaterialTheme.colorScheme.primary
    Column(Modifier.fillMaxWidth().padding(16.dp, 4.dp)) {
        Text(stringResource(R.string.video_frame), style = MaterialTheme.typography.titleSmall)
        Box(Modifier.fillMaxWidth().aspectRatio(picture.width.toFloat() / picture.height)) {
            Image(picture.asImageBitmap(), null, Modifier.fillMaxWidth(), contentScale = ContentScale.FillBounds)
            Canvas(
                Modifier.matchParentSize().pointerInput(scale) {
                    val free = Offset(size.width * (1 - 1 / scale) / 2, size.height * (1 - 1 / scale) / 2)
                    detectDragGestures(onDragEnd = { done(px.floatValue, py.floatValue) }) { _, drag ->
                        if (free.x > 0) px.floatValue = (px.floatValue + drag.x / free.x).coerceIn(-1f, 1f)
                        if (free.y > 0) py.floatValue = (py.floatValue + drag.y / free.y).coerceIn(-1f, 1f)
                    }
                },
            ) {
                val window = Size(size.width / scale, size.height / scale)
                val corner = Offset((size.width - window.width) * (px.floatValue + 1) / 2, (size.height - window.height) * (py.floatValue + 1) / 2)
                drawRect(edge, corner, window, style = Stroke(4.dp.toPx()))
            }
        }
        Text(stringResource(R.string.video_frame_help), style = MaterialTheme.typography.bodySmall)
    }
}

@Composable
fun Reeler(main: Main, uid: String, form: JSONObject, set: (String, Any?) -> Unit) {
    val probe = form.optString("probe")
    LaunchedEffect(probe) {
        if (probe.isNotEmpty()) {
            withContext(Dispatchers.IO) { poster(uid) { setDataSource(probe) } }
            main.revision.intValue++
        }
    }
    val length = form.optDouble("length", 0.0).toFloat()
    if (length >= 1f) Trim(length, form.number("start"), form.number("end")) { start, end ->
        set("start", start)
        set("end", end)
    }
    Framing(uid, form.number("zoom"), form.number("x"), form.number("y")) { x, y ->
        set("x", x)
        set("y", y)
    }
}
