package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp

@Composable
fun Seek(text: String, change: (String) -> Unit, hint: String, modifier: Modifier = Modifier) {
    val t = tones
    var focused by remember { mutableStateOf(false) }
    val shape = RoundedCornerShape(Measure.rControl)
    BasicTextField(
        text,
        change,
        modifier.fillMaxWidth().padding(horizontal = Measure.margin).onFocusChanged { focused = it.isFocused }.semantics { contentDescription = hint }.testTag("seek"),
        singleLine = true,
        textStyle = Type.text.copy(color = t.frost),
        cursorBrush = SolidColor(t.frost),
        decorationBox = { field ->
            Row(
                Modifier.heightIn(min = Measure.touch).background(t.nightHigh, shape)
                    .border(if (focused) 2.dp else Measure.line, if (focused) t.frost else t.nightLine, shape)
                    .padding(start = if (focused) 13.dp else 14.dp, end = 2.dp),
                Arrangement.spacedBy(10.dp),
                Alignment.CenterVertically,
            ) {
                Glyph(Ph.MagnifyingGlass, null, tint = t.mist, size = Measure.iconSmall)
                Box(Modifier.weight(1f)) {
                    if (text.isEmpty()) Text(hint, color = t.mist, style = Type.text)
                    field()
                }
                if (text.isNotEmpty()) Square({ change("") }, Ph.X, stringResource(R.string.clear_search), boxed = false)
            }
        },
    )
}
