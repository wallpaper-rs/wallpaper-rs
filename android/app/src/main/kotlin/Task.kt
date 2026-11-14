package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.exclude
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp

private val HEAD = 196.dp
private val BLOCK_GAP = 6.dp

@Composable
fun TaskPage(foot: (@Composable RowScope.() -> Unit)?, content: @Composable ColumnScope.() -> Unit) =
    Column(Modifier.fillMaxSize().windowInsetsPadding(WindowInsets.ime.exclude(WindowInsets.navigationBars))) {
        Column(Modifier.weight(1f).fillMaxWidth().verticalScroll(rememberScrollState()), content = content)
        foot?.let { Foot(it) }
    }

@Composable
fun Foot(content: @Composable RowScope.() -> Unit) {
    val t = tones
    Column(Modifier.fillMaxWidth().background(t.nightHigh).testTag("foot")) {
        Spacer(Modifier.fillMaxWidth().height(Measure.line).background(t.nightLine))
        Row(
            Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e3),
            Arrangement.spacedBy(Measure.e2),
            Alignment.CenterVertically,
            content,
        )
    }
}

@Composable
fun StripHead(title: String, meta: @Composable () -> Unit) =
    Column(Modifier.fillMaxWidth().heightIn(min = HEAD).padding(start = Measure.margin, end = Measure.margin, bottom = Measure.e5), Arrangement.Bottom) {
        Text(title, style = Type.titleM)
        Box(Modifier.padding(top = 2.dp)) { meta() }
    }

@Composable
fun Block(title: String, text: String?, modifier: Modifier = Modifier, actions: (@Composable ColumnScope.() -> Unit)? = null) =
    Column(modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e5)) {
        Text(title, style = Type.row.copy(fontFamily = RedHat, fontWeight = Type.titleS.fontWeight))
        text?.let { Text(it, Modifier.padding(top = BLOCK_GAP), tones.mist, style = Type.text) }
        actions?.let { Column(Modifier.padding(top = Measure.e4), Arrangement.spacedBy(Measure.e3), content = it) }
    }

@Composable
fun ThinSeam() = Spacer(Modifier.padding(start = Measure.margin, end = Measure.margin, top = Measure.e5).fillMaxWidth().height(Measure.line).background(tones.nightLine))

@Composable
fun Found(name: String, sub: String?, ph: Ph = Ph.HardDrives, modifier: Modifier = Modifier, onClick: (() -> Unit)? = null) {
    val tap = onClick?.let { Modifier.clickable(role = Role.Button, onClick = it) } ?: Modifier
    Row(modifier.fillMaxWidth().then(tap).heightIn(min = Measure.option).padding(horizontal = Measure.margin), Arrangement.spacedBy(Measure.e3), Alignment.CenterVertically) {
        Glyph(ph, null, tint = tones.mist, size = 22.dp)
        Column(Modifier.weight(1f)) {
            Text(name, style = Type.text)
            sub?.let { Text(it, color = tones.mist, style = Type.foot) }
        }
    }
}

@Composable
fun Digits(code: String, modifier: Modifier = Modifier) {
    val plain = code.filter(Char::isDigit)
    val shown = if (plain.length == 6) plain.chunked(3).joinToString(" ") else code
    Text(shown, modifier, style = Type.digits)
}

@Composable
fun TextBox(
    label: String,
    value: String,
    change: (String) -> Unit,
    modifier: Modifier = Modifier,
    example: String? = null,
    problem: String? = null,
    hidden: Boolean = false,
    kind: KeyboardType = KeyboardType.Uri,
    done: () -> Unit = {},
    trailing: @Composable RowScope.() -> Unit = {},
) = Column(modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e2)) {
    val t = tones
    val focus = remember { MutableInteractionSource() }
    val focused by focus.collectIsFocusedAsState()
    val edge = when {
        problem != null -> 2.dp to t.coral
        focused -> 2.dp to t.frost
        else -> Measure.line to t.nightLine
    }
    val shape = RoundedCornerShape(Measure.rControl)
    Text(label, Modifier.padding(bottom = BLOCK_GAP), t.mist, style = Type.foot.copy(fontWeight = Type.tab.fontWeight))
    Row(
        Modifier.fillMaxWidth().heightIn(min = Measure.field).background(t.nightHigh, shape).border(edge.first, edge.second, shape)
            .padding(start = 14.dp, end = Measure.e1),
        Arrangement.spacedBy(2.dp),
        Alignment.CenterVertically,
    ) {
        BasicTextField(
            value,
            change,
            Modifier.weight(1f).semantics { contentDescription = label },
            textStyle = Type.row.copy(color = t.frost),
            singleLine = true,
            cursorBrush = SolidColor(t.frost),
            visualTransformation = if (hidden) PasswordVisualTransformation() else VisualTransformation.None,
            keyboardOptions = KeyboardOptions(keyboardType = if (hidden) KeyboardType.Password else kind, imeAction = ImeAction.Done),
            keyboardActions = KeyboardActions(onDone = { done() }),
            interactionSource = focus,
        )
        trailing()
    }
    (problem ?: example)?.let { Text(it, Modifier.padding(top = BLOCK_GAP), if (problem != null) t.coral else t.mist, style = Type.foot) }
}
