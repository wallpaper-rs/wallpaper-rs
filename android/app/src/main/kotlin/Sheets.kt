package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

private const val SCRIM = 0.62f

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun Sheet(close: () -> Unit, title: String? = null, note: String? = null, content: @Composable ColumnScope.() -> Unit) {
    val t = tones
    ModalBottomSheet(
        onDismissRequest = close,
        shape = RoundedCornerShape(topStart = Measure.rSheet, topEnd = Measure.rSheet),
        containerColor = t.nightHigh,
        contentColor = t.frost,
        scrimColor = t.veiled(SCRIM),
        dragHandle = {
            Box(Modifier.padding(top = 10.dp, bottom = 10.dp).size(Measure.handle, Measure.markHeight).background(t.mistLow, RoundedCornerShape(2.dp)))
        },
    ) {
        Column(Modifier.padding(bottom = Measure.e3)) {
            title?.let { Text(it, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), style = Type.row.copy(fontFamily = RedHat, fontWeight = FontWeight.Bold)) }
            note?.let { Text(it, Modifier.padding(start = Measure.margin, end = Measure.margin, bottom = Measure.e2), t.mist, style = Type.foot) }
            content()
        }
    }
}

@Composable
fun SheetOption(label: String, modifier: Modifier = Modifier, ph: Ph? = null, sub: String? = null, chosen: Boolean = false, onClick: () -> Unit) {
    val t = tones
    Row(
        modifier.fillMaxWidth().clickable(role = Role.RadioButton, onClick = onClick).heightIn(min = Measure.option).padding(horizontal = Measure.margin),
        horizontalArrangement = Arrangement.spacedBy(14.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        ph?.let { Glyph(it, null, tint = t.mist, size = 22.dp) }
        Column(Modifier.weight(1f).padding(vertical = Measure.e2)) {
            Text(label, style = Type.row.copy(fontWeight = if (chosen) FontWeight.SemiBold else FontWeight.Normal))
            sub?.let { Text(it, color = t.mist, style = Type.foot) }
        }
        if (chosen) Glyph(Ph.Check, null, tint = t.moon, size = 22.dp)
    }
}

@Composable
fun Question(
    question: String,
    note: String?,
    confirm: String,
    cancel: String,
    removes: Boolean,
    close: () -> Unit,
    accept: () -> Unit,
) = Sheet(close) {
    Text(question, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), style = Type.row.copy(fontFamily = RedHat, fontWeight = FontWeight.Bold))
    note?.let { Text(it, Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text) }
    Row(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e4), Arrangement.spacedBy(Measure.e2)) {
        QuietAction(close, Modifier.weight(1f)) { Text(cancel) }
        Action(accept, Modifier.weight(1f), kind = if (removes) Kind.Remove else Kind.Main) { Text(confirm) }
    }
}
