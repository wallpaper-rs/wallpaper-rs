package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.saveable.listSaver
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.runtime.toMutableStateList
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

@Composable
fun rememberSelection(): SnapshotStateList<String> =
    rememberSaveable(saver = listSaver({ it.toList() }, { it.toMutableStateList() })) { mutableStateListOf() }

fun SnapshotStateList<String>.toggle(uid: String) {
    if (!remove(uid)) add(uid)
}

@Composable
fun SnapshotStateList<String>.follow(items: List<Item>?) = LaunchedEffect(items) {
    items?.map(Item::uid)?.toSet()?.let { shown -> retainAll(shown) }
}

@Composable
fun Selecting(main: Main, chosen: SnapshotStateList<String>, all: List<String>, playlist: String? = null, bar: Boolean = true) {
    val count = chosen.size
    BackHandler(chosen.isNotEmpty()) { chosen.clear() }
    Row(
        Modifier.fillMaxWidth().background(tones.nightHigh).heightIn(min = BAR).padding(horizontal = 6.dp).testTag("choosing"),
        Arrangement.spacedBy(2.dp),
        Alignment.CenterVertically,
    ) {
        Square(chosen::clear, Ph.X, stringResource(R.string.cancel), boxed = false)
        Text(pluralStringResource(R.plurals.lib_chosen, count, count), Modifier.weight(1f), style = Type.row.copy(fontFamily = RedHat, fontWeight = FontWeight.Bold))
        Square({ chosen.addAll(all.filter { it !in chosen }) }, Ph.Checks, stringResource(R.string.lib_choose_all), boxed = false)
    }
    Box(Modifier.fillMaxWidth().height(Measure.line).background(tones.nightLine))
    if (bar) ChosenBar(main, chosen, playlist, Modifier.padding(Measure.e3))
}

@Composable
fun AddTo(close: () -> Unit, add: (String) -> Unit) {
    val lists by loaded(emptyList<Playlist>()) { playlists() }
    Choose(stringResource(R.string.add_to_playlist), close, lists.map { it.uid to it.name }, stringResource(R.string.no_playlists), pick = add)
}

@Composable
fun Fits(close: () -> Unit, fit: (String) -> Unit) = Choose(stringResource(R.string.fit), close, fitChoices(), "", pick = fit)

@Composable
fun Choose(title: String, close: () -> Unit, choices: List<Pair<String, String>>, empty: String, chosen: String = "", pick: (String) -> Unit) =
    Sheet(close, title, empty.takeIf { choices.isEmpty() && it.isNotEmpty() }) {
        choices.forEach { (value, label) ->
            SheetOption(label, chosen = value == chosen) {
                close()
                pick(value)
            }
        }
    }

@Composable
fun Removing(count: Int, close: () -> Unit, remove: () -> Unit) = Question(
    pluralStringResource(R.plurals.remove_question, count, count),
    null,
    stringResource(R.string.remove),
    stringResource(R.string.cancel),
    true,
    close,
) {
    close()
    remove()
}

private val BAR = 56.dp
