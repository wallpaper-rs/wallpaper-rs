package io.github.wallpaper_rs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

val INTERVALS = listOf(
    "10m" to R.string.every_10m,
    "15m" to R.string.every_15m,
    "30m" to R.string.every_30m,
    "1h" to R.string.every_1h,
    "6h" to R.string.every_6h,
    "1d" to R.string.every_1d,
    "0s" to R.string.never,
)

val Measure.mini: Dp get() = 72.dp
val Measure.rMini: Dp get() = 4.dp

@Composable
fun Playlists(main: Main, load: suspend () -> List<Playlist> = { playlists() }, open: (String) -> Unit) = Box(Modifier.fillMaxSize()) {
    var again by remember { mutableIntStateOf(0) }
    val got by loaded<Result<List<Playlist>>?>(null, main.revision.intValue, again) { runCatching { load() } }
    var naming by remember { mutableStateOf(false) }
    val lists = got?.getOrNull()
    LazyColumn(contentPadding = PaddingValues(bottom = Measure.e6)) {
        item { Header(stringResource(R.string.playlists)) }
        got?.exceptionOrNull()?.let { error ->
            item {
                val context = LocalContext.current
                Problem(stringResource(R.string.lists_unread), stringResource(R.string.lists_unread_why), details = context.told(error)) {
                    QuietAction({ again++ }) { Text(stringResource(R.string.lists_retry)) }
                }
            }
        }
        if (lists?.isEmpty() == true) item {
            Blank(stringResource(R.string.no_playlists), stringResource(R.string.lists_how), Modifier.padding(top = Measure.e10).testTag("lists-empty")) {
                QuietAction({ Jump.to.value = Tab.Library }, Modifier.fillMaxWidth()) {
                    Glyph(Ph.Images, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
                    Text(stringResource(R.string.lists_go_library))
                }
                WordAction({ naming = true }, Modifier.fillMaxWidth()) { Text(stringResource(R.string.new_playlist)) }
            }
        }
        items(lists.orEmpty(), key = Playlist::uid) { list ->
            ListRow(main, list) { open(list.uid) }
            Seam()
        }
        if (!lists.isNullOrEmpty()) item {
            QuietAction({ naming = true }, Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e5)) {
                Glyph(Ph.Plus, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
                Text(stringResource(R.string.new_playlist))
            }
        }
    }
    if (naming) NameSheet(stringResource(R.string.new_playlist), stringResource(R.string.name), "", stringResource(R.string.create), { naming = false }) { name ->
        main.act { ask("create", "name" to name) }
    }
}

@Composable
private fun ListRow(main: Main, list: Playlist, open: () -> Unit) = Column(
    Modifier.fillMaxWidth().clickable(role = Role.Button, onClick = open).testTag("list-${list.uid}"),
) {
    val t = tones
    Row(
        Modifier.fillMaxWidth().heightIn(min = Measure.row).padding(horizontal = Measure.margin, vertical = 10.dp),
        Arrangement.spacedBy(Measure.e3),
        Alignment.CenterVertically,
    ) {
        Mini(main, list.uid)
        Column(Modifier.weight(1f)) {
            Text(list.name, style = Type.row)
            Row(Modifier.padding(top = 2.dp), Arrangement.spacedBy(Measure.e2), Alignment.CenterVertically) {
                if (list.current) Row(horizontalArrangement = Arrangement.spacedBy(Measure.e1), verticalAlignment = Alignment.CenterVertically) {
                    Glyph(Ph.CheckCircle, null, tint = t.moon, lit = true, size = 15.dp)
                    Text(stringResource(R.string.in_use), color = t.frost, style = Type.foot.copy(fontWeight = FontWeight.SemiBold))
                }
                Text(summary(list), color = t.mist, style = Type.foot)
            }
        }
        Glyph(Ph.CaretRight, null, tint = t.mistLow, size = Measure.iconSmall)
    }
    list.away.forEach { AwayLine(it, Modifier.padding(start = Measure.margin, end = Measure.margin, bottom = Measure.e2)) }
}

@Composable
private fun Mini(main: Main, uid: String) {
    val first by loaded<Item?>(null, main.revision.intValue, uid) { fetch("entries", "playlist" to uid).firstOrNull() }
    Cover(first)
}

@Composable
fun every(rules: org.json.JSONObject): String =
    stringResource(INTERVALS.find { it.first == rules.optString("interval") }?.second ?: R.string.every_30m)

@Composable
private fun summary(list: Playlist): String = every(list.rules) + " · " + pluralStringResource(R.plurals.wallpapers, list.count, list.count)

object Jump {
    val to = mutableStateOf<Tab?>(null)
}
