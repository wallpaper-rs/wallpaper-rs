package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp

@Composable
fun Cover(item: Item, modifier: Modifier = Modifier, inUse: Boolean = false, picked: Boolean = false) = Column(modifier.testTag("cover")) {
    Box(Modifier.fillMaxWidth().aspectRatio(RATIO)) {
        Thumb(item, Modifier.fillMaxSize(), picked, flagged = false)
        if (inUse) {
            val shape = RoundedCornerShape(Measure.e1)
            InUse(Modifier.align(Alignment.BottomStart).padding(6.dp).background(tones.night, shape).heightIn(min = 26.dp).padding(start = 6.dp, end = Measure.e2))
        }
    }
    Text(
        item.title,
        Modifier.padding(top = 9.dp),
        tones.frost,
        style = Type.foot.copy(fontFamily = RedHat, fontWeight = FontWeight.Medium),
        maxLines = 2,
        overflow = TextOverflow.Ellipsis,
    )
    if (item.flashing) StateLine(stringResource(R.string.flashes_barred), Tone.Amber, Modifier.padding(top = 2.dp))
}

@Composable
fun Thumb(item: Item, modifier: Modifier = Modifier, picked: Boolean = false, flagged: Boolean = true) {
    val bitmap by thumbnail(item)
    val shape = RoundedCornerShape(Measure.rControl)
    val outline = if (picked) Modifier.border(2.dp, tones.frost, shape) else Modifier
    Box(modifier.then(outline).clip(shape).background(tones.nightHigh)) {
        val kind = stringResource(if (item.video) R.string.kind_video else if (item.scene) R.string.kind_scene else R.string.kind_photo)
        bitmap?.let { Image(it, "$kind ${item.title}".trim(), Modifier.fillMaxSize(), contentScale = ContentScale.Crop) }
        if (flagged && item.flashing) StateLine(stringResource(R.string.flashes_barred), Tone.Amber, Modifier.align(Alignment.TopStart).fillMaxWidth().background(tones.nightHigh).padding(Measure.e1))
        if (item.failing != null) Glyph(Ph.Warning, stringResource(R.string.page_error), Modifier.align(Alignment.TopStart).padding(6.dp), tones.coral)
        val corner = Modifier.align(Alignment.BottomEnd).padding(6.dp)
        if (item.video) Glyph(Ph.Play, null, corner, tones.frost)
        if (item.scene) Glyph(if (item.web) Ph.GlobeSimple else Ph.Sparkle, null, corner, tones.frost)
        if (picked) Glyph(Ph.CheckCircle, stringResource(R.string.lib_chosen_mark), Modifier.align(Alignment.TopEnd).padding(6.dp), tones.frost, lit = true)
    }
}

private const val RATIO = 0.75f
