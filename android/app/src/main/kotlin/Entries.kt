package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.zIndex
import org.json.JSONObject

@Composable
fun Entries(main: Main, uid: String, open: (String) -> Unit, close: () -> Unit) = Column {
    val list by loaded<Playlist?>(null, main.revision.intValue, uid) { playlists().find { it.uid == uid } }
    val found by loaded<List<Item>?>(null, main.revision.intValue, uid) { fetch("entries", "playlist" to uid) }
    val items = found.orEmpty()
    val rules = list?.rules ?: JSONObject()
    val manual = list?.info?.isNull("query") != false
    val keep: (JSONObject) -> Unit = { info -> main.act { ask("save", "playlist" to info) } }
    val save: (String, Any) -> Unit = { key, value -> list?.let { keep(it.ruled(key, value)) } }
    val chosen = rememberSelection()
    var sheet by remember { mutableStateOf<String?>(null) }
    var more by rememberSaveable { mutableStateOf(false) }
    var dragged by remember { mutableStateOf<String?>(null) }
    var shift by remember { mutableFloatStateOf(0f) }
    var row by remember { mutableIntStateOf(1) }
    chosen.follow(found)
    DisposableEffect(uid) { onDispose { main.unpick(choice("playlist", uid)) } }
    if (!chosen.isEmpty()) Selecting(main, chosen, items.map(Item::uid), uid)
    else Row(verticalAlignment = Alignment.CenterVertically) {
        Column(Modifier.weight(1f)) { Header(list?.name.orEmpty(), close) }
        Square({ sheet = NAME }, Ph.PencilSimple, stringResource(R.string.rename_playlist), boxed = false)
    }
    list?.away.orEmpty().forEach { AwayLine(it, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e1)) }
    LazyColumn(Modifier.testTag("list-page"), contentPadding = PaddingValues(bottom = Measure.e4)) {
        if (chosen.isEmpty() && manual && found?.isEmpty() == true) item {
            Blank(stringResource(R.string.list_empty), stringResource(R.string.list_empty_how), Modifier.padding(top = Measure.e8, bottom = Measure.e6).testTag("list-empty")) {
                Action({ Jump.to.value = Tab.Library }, Modifier.fillMaxWidth()) {
                    Glyph(Ph.Images, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
                    Text(stringResource(R.string.lists_go_library))
                }
            }
        }
        if (chosen.isEmpty()) item {
            Setting(stringResource(R.string.change), value = every(rules), onClick = { sheet = EVERY })
            Toggle(stringResource(R.string.shuffle), rules.optString("order") == "shuffle") { save("order", if (it) "shuffle" else "sequential") }
            Toggle(stringResource(R.string.skip_videos), rules.optBoolean("skip_videos_on_battery")) { save("skip_videos_on_battery", it) }
            Toggle(stringResource(R.string.synced_list), rules.optBoolean("synced")) { save("synced", it) }
            Schedule(rules, save)
            list?.let { Smart(it, keep) }
            Row(Modifier.padding(horizontal = Measure.margin, vertical = Measure.e4), Arrangement.spacedBy(Measure.e2), Alignment.CenterVertically) {
                Action({ main.act(false) { ask("pick", "choice" to choice("playlist", uid)); main.setWallpaper() } }, enabled = items.isNotEmpty()) {
                    Text(stringResource(R.string.apply))
                }
                WordAction({ sheet = DELETE }) { Text(stringResource(R.string.delete), color = tones.coral) }
            }
            if (items.isNotEmpty()) Group(stringResource(R.string.list_wallpapers))
        }
        itemsIndexed(items, key = { _, item -> item.uid }) { index, item ->
            val moving = dragged == item.uid
            val reset = { dragged = null; shift = 0f }
            val drag = Dragging({ dragged = item.uid }, { shift += it }, {
                val to = landing(index, shift, List(items.size) { row })
                reset()
                if (to != index) main.act { ask("move", "playlist" to uid, "from" to index, "to" to to) }
            }, reset)
            val place = Modifier.onSizeChanged { row = it.height.coerceAtLeast(1) }.zIndex(if (moving) 1f else 0f).graphicsLayer { translationY = if (moving) shift else 0f }
            val tap = { if (chosen.isEmpty()) open(item.uid) else chosen.toggle(item.uid) }
            Listed(item, chosen.isNotEmpty(), item.uid in chosen, manual, place, tap, { chosen.toggle(item.uid) }, { take(main, uid, item.uid) }, drag)
        }
        if (chosen.isEmpty()) item {
            Fold(stringResource(R.string.list_more), more, Modifier.padding(top = Measure.e2)) { more = !more }
            if (more) {
                Toggle(stringResource(R.string.resume_list), rules.optBoolean("resume", true)) { save("resume", it) }
                Fade(rules, save)
            }
        }
    }
    val current = list
    when {
        current == null -> Unit
        sheet == NAME -> NameSheet(stringResource(R.string.rename_playlist), stringResource(R.string.name), current.name, stringResource(R.string.save_name), { sheet = null }) {
            keep(current.edited("name", it))
        }
        sheet == EVERY -> Sheet({ sheet = null }, stringResource(R.string.change)) {
            INTERVALS.forEach { (value, label) ->
                SheetOption(stringResource(label), chosen = rules.optString("interval", "30m") == value) {
                    sheet = null
                    save("interval", value)
                }
            }
        }
        sheet == DELETE -> Question(
            stringResource(R.string.list_delete_ask, current.name), stringResource(R.string.list_delete_note),
            stringResource(R.string.delete), stringResource(R.string.cancel), true, { sheet = null },
        ) {
            sheet = null
            main.act {
                ask("delete", "playlist" to uid)
                close()
            }
        }
    }
}

private const val NAME = "name"
private const val EVERY = "every"
private const val DELETE = "delete"

private fun take(main: Main, playlist: String, item: String) = main.act { ask("take", "playlist" to playlist, "items" to listOf(item)) }
