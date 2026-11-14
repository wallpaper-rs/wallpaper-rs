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
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import java.text.Normalizer

fun folded(text: String) = Normalizer.normalize(text, Normalizer.Form.NFD).replace(MARKS, "").lowercase().trim()

fun finds(query: String, label: String) = folded(query).let { it.isEmpty() || it in folded(label) }

private val MARKS = Regex("\\p{Mn}+")

@Composable
fun Finder(query: String, hint: String, modifier: Modifier = Modifier, change: (String) -> Unit) {
    val t = tones
    val shape = RoundedCornerShape(Measure.rControl)
    val focus = LocalFocusManager.current
    Row(
        modifier.fillMaxWidth().padding(horizontal = Measure.margin).heightIn(min = Measure.touch)
            .background(t.nightHigh, shape).border(Measure.line, t.nightLine, shape).padding(start = 14.dp),
        Arrangement.spacedBy(10.dp),
        Alignment.CenterVertically,
    ) {
        Glyph(Ph.MagnifyingGlass, null, tint = t.mist, size = Measure.iconSmall)
        Box(Modifier.weight(1f), contentAlignment = Alignment.CenterStart) {
            if (query.isEmpty()) Text(hint, color = t.mist, style = Type.text)
            BasicTextField(
                query,
                change,
                Modifier.fillMaxWidth().testTag("finder").semantics { contentDescription = hint },
                textStyle = Type.text.copy(color = t.frost),
                singleLine = true,
                cursorBrush = SolidColor(t.moon),
                keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
                keyboardActions = KeyboardActions(onSearch = { focus.clearFocus() }),
            )
        }
        if (query.isNotEmpty()) Square({ change("") }, Ph.X, stringResource(R.string.clear_search), boxed = false)
    }
}
