package io.github.wallpaper_rs

import android.graphics.ImageDecoder
import android.graphics.drawable.AnimatedImageDrawable
import android.widget.ImageView
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.isSpecified
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import java.io.File

@Composable
fun Header(title: String, back: (() -> Unit)? = null) = if (back == null) {
    Text(title, Modifier.padding(start = Measure.margin, end = Measure.margin, top = Measure.e4, bottom = Measure.e3), style = Type.titleM)
} else {
    Row(Modifier.heightIn(min = Measure.option).padding(horizontal = 6.dp), Arrangement.spacedBy(Measure.e1), Alignment.CenterVertically) {
        Square(back, Ph.ArrowLeft, stringResource(R.string.back), boxed = false)
        Text(title, Modifier.weight(1f), style = Type.titleS)
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Choices(label: String?, options: List<Pair<String, String>>, chosen: String, icons: Map<String, Ph> = emptyMap(), choose: (String) -> Unit) = Column {
    label?.let { Group(it) }
    val t = tones
    FlowRow(Modifier.padding(horizontal = Measure.margin, vertical = Measure.e1), horizontalArrangement = Arrangement.spacedBy(Measure.e2)) {
        options.forEach { (value, text) ->
            val on = chosen == value
            FilterChip(
                on,
                { choose(value) },
                { Text(text, style = Type.text) },
                leadingIcon = icons[value]?.let { { Glyph(it, null, size = Measure.iconSmall) } },
                shape = RoundedCornerShape(Measure.rControl),
                colors = FilterChipDefaults.filterChipColors(
                    containerColor = t.nightHigh,
                    labelColor = t.frost,
                    iconColor = t.frost,
                    selectedContainerColor = t.frost,
                    selectedLabelColor = t.night,
                    selectedLeadingIconColor = t.night,
                ),
                border = BorderStroke(Measure.line, if (on) t.frost else t.nightLine),
            )
        }
    }
}

@Composable
fun Toggle(label: String, checked: Boolean, enabled: Boolean = true, change: (Boolean) -> Unit) = Setting(
    label,
    Modifier.toggleable(value = checked, enabled = enabled, role = Role.Switch, onValueChange = change),
    enabled = enabled,
    trailing = { Flip(checked) },
)

@Composable
fun Note(text: String, color: Color = Color.Unspecified) =
    Text(text, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e1), if (color.isSpecified) color else tones.mist, style = Type.foot)

@Composable
fun Animated(path: String, modifier: Modifier = Modifier) = AndroidView({ ImageView(it) }, modifier) { view ->
    if (view.tag != path) {
        view.tag = path
        val drawable = runCatching { ImageDecoder.decodeDrawable(ImageDecoder.createSource(File(path))) }.getOrNull()
        view.setImageDrawable(drawable)
        (drawable as? AnimatedImageDrawable)?.start()
    }
}
