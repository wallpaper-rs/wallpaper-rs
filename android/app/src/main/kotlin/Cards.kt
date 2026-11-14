package io.github.wallpaper_rs

import androidx.compose.animation.core.animateDpAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.snapping.rememberSnapFlingBehavior
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

private val WIDE = 200.dp
private val NARROW = 116.dp
private const val SHELF_RATIO = 1.6f

@Composable
fun KindTab(label: String, on: Boolean, pick: () -> Unit) = Column(
    Modifier.selectable(on, role = Role.Tab, onClick = pick).heightIn(min = Measure.touch),
    Arrangement.Center,
) {
    Text(label, color = if (on) tones.frost else tones.mist, style = Type.tab)
    Box(Modifier.padding(top = 6.dp).width(18.dp).height(3.dp).background(if (on) tones.moon else Color.Transparent, RoundedCornerShape(2.dp)))
}

@Composable
fun Cards(main: Main, items: List<Item>, inUse: String?, wide: Dp, open: (String) -> Unit) {
    val state = rememberLazyListState()
    val focused by remember(state) {
        derivedStateOf {
            val first = state.layoutInfo.visibleItemsInfo.firstOrNull()
            when {
                first == null -> 0
                -first.offset > first.size / 2 -> first.index + 1
                else -> first.index
            }
        }
    }
    LazyRow(
        Modifier.fillMaxWidth().testTag("shelf"),
        state = state,
        contentPadding = PaddingValues(start = Measure.margin, end = (wide - WIDE - Measure.margin).coerceAtLeast(Measure.margin)),
        horizontalArrangement = Arrangement.spacedBy(Measure.e3),
        verticalAlignment = Alignment.Top,
        flingBehavior = rememberSnapFlingBehavior(state),
    ) {
        itemsIndexed(items, key = { _, item -> item.uid }) { index, item -> Card(main, item, index == focused, item.uid == inUse) { open(item.uid) } }
    }
}

@Composable
private fun Card(main: Main, item: Item, focused: Boolean, inUse: Boolean, open: () -> Unit) {
    val width by animateDpAsState(if (focused) WIDE else NARROW, label = "card")
    Column(Modifier.width(width).testTag(if (focused) "focused" else "cover")) {
        val shape = RoundedCornerShape(Measure.rControl)
        val ring = if (focused) Modifier.border(2.dp, tones.frost, shape) else Modifier
        Box(Modifier.fillMaxWidth().aspectRatio(SHELF_RATIO).then(ring).clickable(onClick = open)) {
            Thumb(item, Modifier.fillMaxSize().padding(if (focused) 3.dp else 0.dp), flagged = false)
            if (inUse) InUse(Modifier.align(Alignment.BottomStart).padding(6.dp).background(tones.night, RoundedCornerShape(Measure.e1)).heightIn(min = 26.dp).padding(start = 6.dp, end = Measure.e2))
        }
        if (!focused) return@Column
        Text(item.title, Modifier.padding(top = Measure.e3), tones.frost, style = Type.row, maxLines = 1, overflow = TextOverflow.Ellipsis)
        Text(if (inUse) stringResource(R.string.lib_stays) else facts(item), Modifier.padding(top = 2.dp), tones.mist, style = Type.foot, maxLines = 2)
        if (item.flashing) StateLine(stringResource(R.string.flashes_barred), Tone.Amber, Modifier.padding(top = 2.dp))
        if (!inUse) Action({ main.act(false) { ask("pick", "choice" to choice("item", item.uid)); main.setWallpaper() } }, Modifier.padding(top = Measure.e3).fillMaxWidth().testTag("shelf_apply"), enabled = !item.flashing) {
            Glyph(Ph.Image, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
            Text(stringResource(R.string.apply))
        }
    }
}
