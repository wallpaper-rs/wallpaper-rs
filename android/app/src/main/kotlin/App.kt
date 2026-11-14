package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue

enum class Aside { Sources, Account, Connect, More, Print }

private fun Aside.back(): Aside? = when (this) {
    Aside.Connect, Aside.More -> Aside.Account
    Aside.Print -> Aside.More
    else -> null
}

@Composable
fun App(main: Main) = Theme {
    var tab by rememberSaveable { mutableStateOf(Tab.Library) }
    var item by rememberSaveable { mutableStateOf<String?>(null) }
    var playlist by rememberSaveable { mutableStateOf<String?>(null) }
    var aside by rememberSaveable { mutableStateOf<Aside?>(null) }
    var printFrom by rememberSaveable { mutableStateOf(Aside.More) }
    val desk = remember(main) { Desk(main) }
    BackHandler(item != null || playlist != null || aside != null || tab != Tab.Library) {
        when {
            item != null -> item = null
            aside != null -> aside = if (aside == Aside.Print) printFrom else aside?.back()
            playlist != null -> playlist = null
            else -> tab = Tab.Library
        }
    }
    val jump = Jump.to.value
    LaunchedEffect(jump) {
        jump?.let {
            tab = it
            playlist = null
            Jump.to.value = null
        }
    }
    if (main.placing.value) Placing(main)
    val (shown, list, away) = Triple(item, playlist, aside)
    val veil = when {
        shown == null && away == Aside.Account -> Veil.Strip
        shown != null || away != null -> Veil.Panel
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
            away == Aside.Account -> Account(desk, { aside = it }, { aside = null; tab = Tab.Library }) { aside = null }
            away == Aside.Connect -> Connect(desk, { printFrom = Aside.Connect; aside = it }, { aside = Aside.Account }) { aside = Aside.Account }
            away == Aside.More -> More(desk, { printFrom = Aside.More; aside = it }) { aside = Aside.Account }
            away == Aside.Print -> Print(desk) { aside = printFrom }
            tab == Tab.Library -> Library(main, { aside = Aside.Sources }) { item = it }
            tab == Tab.Lists && list != null -> Entries(main, list, { item = it }) { playlist = null }
            tab == Tab.Lists -> Playlists(main) { playlist = it }
            else -> Settings(main) { aside = Aside.Account }
        }
    }
}
