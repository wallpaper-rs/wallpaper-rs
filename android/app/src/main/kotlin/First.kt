package io.github.wallpaper_rs

import android.app.WallpaperManager
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

private const val SHOWN = ""

@Composable
fun FirstRun(main: Main) {
    var empty by rememberSaveable { mutableStateOf<Boolean?>(null) }
    var offered by rememberSaveable { mutableStateOf<String?>(null) }
    var asking by rememberSaveable { mutableStateOf<String?>(null) }
    val busy = main.importing.intValue > 0
    LaunchedEffect(Unit) {
        if (empty == null) empty = runCatching { fetch("items", "query" to "").isEmpty() }.getOrDefault(false)
    }
    LaunchedEffect(main.revision.intValue, busy, empty) {
        if (busy || offered != null || empty != true) return@LaunchedEffect
        val items = runCatching { fetch("items", "query" to "") }.getOrNull() ?: return@LaunchedEffect
        val first = items.firstOrNull { !it.flashing } ?: return@LaunchedEffect
        empty = false
        if (nowOn() != null) return@LaunchedEffect
        runCatching { ask("pick", "choice" to choice("item", first.uid)) }
        offered = first.uid
        if (main.ours()) main.place(HOME)
    }
    offered?.takeIf { it != SHOWN }?.let { uid ->
        Offer(main, uid, { main.unpick(choice("item", uid)); offered = SHOWN }) {
            offered = SHOWN
            asking = uid
        }
    }
    asking?.let { uid -> AsksSheet(main, uid) { asking = null } }
}

@Composable
private fun Offer(main: Main, uid: String, close: () -> Unit, chosen: () -> Unit) {
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    val item by loaded<Item?>(null, uid) { (ask("item", "uid" to uid, "languages" to languages) as? JSONObject)?.let(::Item) }
    val live = main.ours()
    val note = stringResource(if (live) R.string.first_set_home else R.string.first_set_confirm)
    Sheet(close, stringResource(R.string.first_set, item?.title.orEmpty()), note) {
        Text(stringResource(R.string.first_where), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), tones.mist, style = Type.group)
        val targets = if (separate()) TARGETS else TARGETS.filter { it.first == BOTH }
        targets.forEach { (target, label, ph) ->
            SheetOption(stringResource(label), Modifier.testTag("first_$target"), ph, chosen = live && target == HOME) {
                chosen()
                main.place(target)
            }
        }
    }
}

fun Main.ours() = runCatching { WallpaperManager.getInstance(this).wallpaperInfo?.packageName == packageName }.getOrDefault(false)
