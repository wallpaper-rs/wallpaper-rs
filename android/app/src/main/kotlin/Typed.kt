package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
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
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp

@Composable
fun Typed(
    value: String,
    change: (String) -> Unit,
    label: String,
    modifier: Modifier = Modifier,
    hint: String? = null,
    focus: FocusRequester? = null,
    done: () -> Unit = {},
) {
    val t = tones
    val press = remember { MutableInteractionSource() }
    val focused by press.collectIsFocusedAsState()
    val edge = if (focused) 2.dp else Measure.line
    BasicTextField(
        value,
        change,
        modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e2).then(focus?.let { Modifier.focusRequester(it) } ?: Modifier),
        singleLine = true,
        textStyle = Type.row.copy(color = t.frost),
        cursorBrush = SolidColor(t.frost),
        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
        keyboardActions = KeyboardActions(onDone = { done() }),
        interactionSource = press,
        decorationBox = { field ->
            Column {
                Text(label, Modifier.padding(bottom = 6.dp), t.mist, style = Type.foot.copy(fontWeight = FontWeight.Medium))
                Row(
                    Modifier.fillMaxWidth().heightIn(min = Measure.field).background(t.nightHigh, RoundedCornerShape(Measure.rControl))
                        .border(edge, if (focused) t.frost else t.nightLine, RoundedCornerShape(Measure.rControl))
                        .padding(start = 14.dp - edge + Measure.line, end = Measure.e1),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Box(Modifier.weight(1f)) {
                        if (value.isEmpty()) hint?.let { Text(it, color = t.mist, style = Type.row) }
                        field()
                    }
                }
            }
        },
    )
}

@Composable
fun NameSheet(title: String, label: String, start: String, confirm: String, close: () -> Unit, done: (String) -> Unit) {
    var text by remember(start) { mutableStateOf(start) }
    val focus = remember { FocusRequester() }
    val submit = {
        if (text.isNotBlank()) {
            close()
            done(text.trim())
        }
    }
    Sheet(close, title) {
        Typed(text, { text = it }, label, focus = focus, done = submit)
        Row(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e4), Arrangement.spacedBy(Measure.e2, Alignment.End)) {
            QuietAction(close) { Text(stringResource(R.string.cancel)) }
            Action({ submit() }, enabled = text.isNotBlank()) { Text(confirm) }
        }
    }
    LaunchedEffect(Unit) { runCatching { focus.requestFocus() } }
}
