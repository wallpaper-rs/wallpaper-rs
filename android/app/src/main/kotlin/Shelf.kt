package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource

val SHELF_KINDS = listOf(R.string.all to "", R.string.images to "kind:image", R.string.videos to "kind:video")

@Composable
fun Shelf(main: Main, sources: () -> Unit, explore: () -> Unit, all: (Boolean) -> Unit, open: (String) -> Unit) = BoxWithConstraints(Modifier.fillMaxSize()) {
    var tab by rememberSaveable { mutableIntStateOf(0) }
    var adding by remember { mutableStateOf(false) }
    val kind = SHELF_KINDS[tab].second
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    val context = LocalContext.current
    val revision = main.revision.intValue
    val items by loaded<List<Item>?>(null, revision, kind, languages) { fetch("items", "query" to kind, "languages" to languages) }
    val now by loaded<Showing?>(null, revision, languages) { showing(languages) }
    val linked by loaded<String?>(null, revision) { linkedName(Sync.status(), context.online()) }
    val where = remember(now) { context.screens() }
    val bare = tab == 0 && items?.isEmpty() == true
    val wide = maxWidth
    BackHandler(tab != 0) { tab = 0 }
    Column(
        Modifier.fillMaxWidth().verticalScroll(rememberScrollState()).heightIn(min = maxHeight).testTag("shelf_page"),
        verticalArrangement = if (bare) Arrangement.SpaceBetween else Arrangement.Top,
    ) {
        Column(Modifier.testTag("shelf_top")) {
            Top(linked, { all(true) }, { adding = true })
            val current = now
            when {
                current != null -> Now(current, where, { open(current.item.uid) }) { loved ->
                    main.act { ask("favorite", "uids" to listOf(current.item.uid), "favorite" to loved) }
                }
                bare -> Unit
                else -> Header(stringResource(R.string.library))
            }
            Importing(main)
            AwayNotes(main)
            if (bare) Empty(main, Hollow.LIBRARY, sources = explore)
        }
        if (bare) Bundled(main)
        else Column(Modifier.padding(bottom = Measure.e4).testTag("shelf_kinds")) {
            Kinds(tab, { tab = it }) { all(false) }
            val shown = items
            when {
                shown == null -> Unit
                shown.isEmpty() -> Empty(main, hollow(kind, ""), sources = explore)
                else -> Cards(main, shown, now?.item?.uid, wide, open)
            }
        }
    }
    if (adding) Adding(main, sources) { adding = false }
}

@Composable
fun Kinds(tab: Int, pick: (Int) -> Unit, all: () -> Unit) = Row(
    Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.e2, bottom = Measure.e2),
    Arrangement.spacedBy(Measure.e4),
    Alignment.CenterVertically,
) {
    SHELF_KINDS.forEachIndexed { index, (label, _) -> KindTab(stringResource(label), index == tab) { pick(index) } }
    Box(Modifier.weight(1f))
    WordAction(all, Modifier.testTag("see_all")) { Text(stringResource(R.string.lib_see_all)) }
}
