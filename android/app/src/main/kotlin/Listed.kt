package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.gestures.detectVerticalDragGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

class Dragging(val start: () -> Unit, val move: (Float) -> Unit, val end: () -> Unit, val cancel: () -> Unit)

@Composable
fun Cover(item: Item?, picked: Boolean = false) {
    val t = tones
    val shape = RoundedCornerShape(Measure.rMini)
    val edge = if (picked) Modifier.border(2.dp, t.frost, shape) else Modifier
    Box(Modifier.width(Measure.mini).aspectRatio(16f / 9f).clip(shape).background(t.nightHigh).then(edge)) {
        item?.let { shown ->
            val bitmap by thumbnail(shown)
            bitmap?.let { Image(it, null, Modifier.fillMaxSize(), contentScale = ContentScale.Crop) }
        }
    }
}

@Composable
fun Listed(item: Item, choosing: Boolean, picked: Boolean, manual: Boolean, modifier: Modifier, tap: () -> Unit, hold: () -> Unit, take: () -> Unit, drag: Dragging) {
    val t = tones
    val dragged by rememberUpdatedState(drag)
    Row(
        modifier.fillMaxWidth().combinedClickable(onLongClick = hold, onClick = tap).heightIn(min = Measure.row).padding(start = Measure.margin, end = Measure.e1),
        Arrangement.spacedBy(Measure.e3),
        Alignment.CenterVertically,
    ) {
        Cover(item, picked)
        Text(item.title, Modifier.weight(1f).padding(vertical = Measure.e2), style = Type.row)
        when {
            choosing -> Glyph(Ph.CheckCircle, null, Modifier.padding(Measure.e3), if (picked) t.moon else t.mistLow, lit = picked)
            manual -> {
                Square(take, Ph.X, stringResource(R.string.take_out), boxed = false)
                Glyph(Ph.DotsSixVertical, stringResource(R.string.drag_to_move), Modifier.padding(Measure.e3).pointerInput(item.uid) {
                    detectVerticalDragGestures(onDragStart = { dragged.start() }, onDragEnd = { dragged.end() }, onDragCancel = { dragged.cancel() }) { change, amount ->
                        change.consume()
                        dragged.move(amount)
                    }
                }, t.mist)
            }
        }
    }
}
