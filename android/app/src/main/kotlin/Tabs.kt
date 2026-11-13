package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.navigationBars
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Badge
import androidx.compose.material3.BadgedBox
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

enum class Tab(val label: Int, val ph: Ph) {
    Library(R.string.library, Ph.Images),
    Lists(R.string.playlists, Ph.Playlist),
    Settings(R.string.settings, Ph.GearSix),
}

@Composable
fun Tabs(current: Tab, updates: Int, choose: (Tab) -> Unit) {
    val t = tones
    Column(Modifier.fillMaxWidth().background(t.nightHigh).windowInsetsPadding(WindowInsets.navigationBars).testTag("tabs")) {
        Spacer(Modifier.fillMaxWidth().height(Measure.line).background(t.nightLine))
        Row(Modifier.fillMaxWidth().padding(horizontal = Measure.e3).selectableGroup()) {
            Tab.entries.forEach { tab -> Cell(tab, tab == current, if (tab == Tab.Library) updates else 0) { choose(tab) } }
        }
    }
}

@Composable
private fun androidx.compose.foundation.layout.RowScope.Cell(tab: Tab, on: Boolean, updates: Int, pick: () -> Unit) {
    val t = tones
    val ink = if (on) t.frost else t.mist
    Box(
        Modifier.weight(1f).heightIn(min = Measure.bar).selectable(on, role = Role.Tab, onClick = pick),
        contentAlignment = Alignment.Center,
    ) {
        if (on) Spacer(Modifier.align(Alignment.TopCenter).size(Measure.mark, Measure.markHeight).background(t.moon, RoundedCornerShape(bottomStart = 2.dp, bottomEnd = 2.dp)))
        Column(Modifier.padding(vertical = Measure.e2), Arrangement.spacedBy(2.dp), Alignment.CenterHorizontally) {
            val name = stringResource(R.string.repo_updates)
            BadgedBox({
                if (updates > 0) Badge(Modifier.semantics { contentDescription = name }, t.moon, t.night) { Text("$updates", style = Type.foot) }
            }) { Glyph(tab.ph, null, lit = on, tint = ink) }
            Text(stringResource(tab.label), color = ink, style = Type.tab.copy(fontWeight = if (on) FontWeight.Bold else FontWeight.Medium))
        }
    }
}
