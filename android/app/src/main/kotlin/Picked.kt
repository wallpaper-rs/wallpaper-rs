package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay

class Made(val uid: String, val name: String)

@Composable
fun ChosenBar(main: Main, chosen: SnapshotStateList<String>, playlist: String?, modifier: Modifier = Modifier, rotated: (Made) -> Unit = {}) {
    var sheet by remember { mutableStateOf("") }
    val uids = chosen.toList()
    val done = { work: suspend () -> Unit -> main.act { work().also { chosen.clear() } } }
    val shape = RoundedCornerShape(Measure.rNotice)
    val rotation = stringResource(R.string.rotation_name)
    Row(
        modifier.fillMaxWidth().background(tones.nightHigh, shape).border(Measure.line, tones.nightLine, shape).padding(6.dp).testTag("chosen_bar"),
        Arrangement.spacedBy(Measure.e1),
        Alignment.CenterVertically,
    ) {
        if (playlist == null) Deed(Ph.Playlist, stringResource(R.string.lib_to_list)) { sheet = "add" }
        Deed(Ph.Heart, stringResource(R.string.favorite)) { done { ask("favorite", "uids" to uids, "favorite" to true) } }
        Deed(Ph.DotsThree, stringResource(R.string.lib_more)) { sheet = "more" }
        if (playlist == null) Action({
            main.act(false) {
                val uid = ask("rotate", "name" to rotation, "items" to uids, "every" to HOURLY) as String
                val name = playlists().firstOrNull { it.uid == uid }?.name ?: rotation
                chosen.clear()
                rotated(Made(uid, name))
            }
        }, Modifier.weight(1f)) {
            Glyph(Ph.Shuffle, null, Modifier.padding(end = Measure.e2), tones.night, size = Measure.iconSmall)
            Text(stringResource(R.string.lib_rotate_these), maxLines = 1)
        }
    }
    val close = { sheet = "" }
    when (sheet) {
        "add" -> AddTo(close) { list -> done { ask("add", "playlist" to list, "items" to uids) } }
        "more" -> Sheet(close) {
            if (playlist != null) SheetOption(stringResource(R.string.take_out), ph = Ph.Minus) {
                close()
                done { ask("take", "playlist" to playlist, "items" to uids) }
            }
            SheetOption(stringResource(R.string.unfavorite), ph = Ph.Heart) {
                close()
                done { ask("favorite", "uids" to uids, "favorite" to false) }
            }
            SheetOption(stringResource(R.string.fit), ph = Ph.SlidersHorizontal) { sheet = "fit" }
            if (playlist == null) SheetOption(stringResource(R.string.remove), ph = Ph.Trash, removes = true) { sheet = "remove" }
        }
        "fit" -> Fits(close) { fit -> main.fit(uids, fit, chosen::clear) }
        "remove" -> Removing(uids.size, close) { done { ask("remove", "uids" to uids) } }
    }
}

@Composable
private fun Deed(ph: Ph, label: String, onClick: () -> Unit) = Column(
    Modifier.clip(RoundedCornerShape(Measure.rControl)).clickable(role = Role.Button, onClick = onClick)
        .defaultMinSize(minWidth = 58.dp, minHeight = 52.dp).padding(horizontal = 6.dp, vertical = 2.dp),
    Arrangement.spacedBy(2.dp, Alignment.CenterVertically),
    Alignment.CenterHorizontally,
) {
    Glyph(ph, null, tint = tones.frost, size = Measure.iconSquare)
    Text(label, color = tones.frost, style = Type.foot.copy(fontWeight = FontWeight.Medium))
}

@Composable
fun Rotated(main: Main, made: Made, modifier: Modifier = Modifier, gone: () -> Unit) {
    var changing by remember(made) { mutableStateOf(false) }
    val asking = main.placing.value
    LaunchedEffect(made, changing, asking) {
        if (changing || asking) return@LaunchedEffect
        delay(NOTICE_MS)
        gone()
    }
    Tell(stringResource(R.string.lib_rotation_made, made.name), modifier.testTag("rotated"), Ph.CheckCircle) {
        WordAction({ changing = true }) { Text(stringResource(R.string.lib_change_interval)) }
    }
    if (changing) {
        val intervals = INTERVALS.filter { it.first != "0s" }.map { (value, label) -> value to stringResource(label) }
        Choose(stringResource(R.string.rotate_every), gone, intervals, "", HOURLY) { every ->
            main.act {
                playlists().firstOrNull { it.uid == made.uid }?.let { ask("save", "playlist" to it.ruled("interval", every)) }
            }
        }
    }
}

const val HOURLY = "1h"
private const val NOTICE_MS = 8000L
