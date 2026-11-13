package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue

enum class Aside { Sources, Account }

@Composable
fun App(main: Main) = Theme {
    var tab by rememberSaveable { mutableStateOf(Tab.Library) }
    var item by rememberSaveable { mutableStateOf<String?>(null) }
    var playlist by rememberSaveable { mutableStateOf<String?>(null) }
    var aside by rememberSaveable { mutableStateOf<Aside?>(null) }
    BackHandler(item != null || playlist != null || aside != null || tab != Tab.Library) {
        when {
            item != null -> item = null
            aside != null -> aside = null
            playlist != null -> playlist = null
            else -> tab = Tab.Library
        }
    }
    if (main.placing.value) Placing(main)
    val (shown, list, away) = Triple(item, playlist, aside)
    val veil = when {
        shown != null || away != null -> Veil.Panel
        tab == Tab.Settings -> Veil.Strip
        else -> Veil.Shelf
    }
    val bar: (@Composable () -> Unit)? = if (shown == null && away == null) {
        {
            Tabs(tab, Updates.count.intValue) {
                tab = it
                playlist = null
            }
        }
    } else {
        null
    }
    Shell(main, veil, bar) {
        when {
            shown != null -> Detail(main, shown, { item = it }) { item = null }
            away == Aside.Sources -> Sources(main, { aside = null }) {
                main.search.value = it
                aside = null
                tab = Tab.Library
            }
            away == Aside.Account -> Account(main) { aside = null }
            tab == Tab.Library -> Library(main, { aside = Aside.Sources }) { item = it }
            tab == Tab.Lists && list != null -> Entries(main, list, { item = it }) { playlist = null }
            tab == Tab.Lists -> Playlists(main) { playlist = it }
            else -> Settings(main) { aside = Aside.Account }
        }
    }
}
