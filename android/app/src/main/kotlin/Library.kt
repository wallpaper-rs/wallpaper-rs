package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.GridItemSpan
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.lazy.grid.rememberLazyGridState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.layout.layout
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay

private val FILTERS = listOf(
    R.string.all to "",
    R.string.images to "kind:image",
    R.string.videos to "kind:video",
    R.string.scenes to SCENES,
    R.string.webs to "kind:web",
    R.string.favorites to "fav:",
)

@Composable
fun Library(main: Main, sources: () -> Unit, open: (String) -> Unit) = Box(Modifier.fillMaxSize()) {
    var filter by rememberSaveable { mutableIntStateOf(0) }
    var text by rememberSaveable { mutableStateOf("") }
    var searching by rememberSaveable { mutableStateOf(false) }
    var adding by remember { mutableStateOf(false) }
    var made by remember { mutableStateOf<Made?>(null) }
    var searched by remember { mutableStateOf(text) }
    LaunchedEffect(text) {
        delay(SEARCH_MS)
        searched = text.trim()
    }
    LaunchedEffect(Unit) {
        main.search.value?.let { found ->
            text = found
            searched = found
            filter = 0
            searching = true
            main.search.value = null
        }
    }
    val chosen = rememberSelection()
    val kind = FILTERS[filter].second
    val query = listOf(kind, searched).filter(String::isNotEmpty).joinToString(" ")
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    val context = LocalContext.current
    val revision = main.revision.intValue
    val items by loaded<List<Item>?>(null, revision, query, languages) { fetch("items", "query" to query, "languages" to languages) }
    val now by loaded<Showing?>(null, revision, languages) { showing(languages) }
    val linked by loaded<String?>(null, revision) { linkedName(Sync.status(), context.online()) }
    val where = remember(now) { context.screens() }
    chosen.follow(items)
    val reset = {
        text = ""
        searched = ""
        filter = 0
        searching = false
    }
    BackHandler(text.isNotEmpty() || filter != 0 || searching, reset)
    val plain = query.isEmpty() && !searching && text.isEmpty()
    val grid = rememberLazyGridState()
    val focus = remember { FocusRequester() }
    LaunchedEffect(searching) {
        if (!searching) return@LaunchedEffect
        grid.scrollToItem(0)
        runCatching { focus.requestFocus() }
    }
    Column {
        if (chosen.isEmpty()) Top(linked, { if (searching && text.isEmpty()) searching = false else searching = true }, { adding = true }, searching)
        else Selecting(main, chosen, items.orEmpty().map(Item::uid), bar = false)
        val shown = items.orEmpty()
        val inUse = now?.item?.uid
        LazyVerticalGrid(
            state = grid,
            columns = GridCells.Adaptive(112.dp),
            contentPadding = PaddingValues(Measure.margin, 0.dp, Measure.margin, 96.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            item(key = "head", span = { GridItemSpan(maxLineSpan) }) {
                Column(Modifier.bleed(Measure.margin)) {
                    val current = now
                    when {
                        plain && current != null -> Now(current, where, { open(current.item.uid) }) { loved ->
                            main.act { ask("favorite", "uids" to listOf(current.item.uid), "favorite" to loved) }
                        }
                        plain -> Header(stringResource(R.string.library))
                        else -> Seek(text, { text = it }, stringResource(R.string.lib_search_library), Modifier.padding(top = Measure.e2, bottom = Measure.e2).focusRequester(focus))
                    }
                    Choices(null, FILTERS.map { (label, query) -> query to stringResource(label) }, kind) { picked ->
                        filter = FILTERS.indexOfFirst { it.second == picked }
                    }
                    if (kind == SCENES || kind == "kind:web") Row(Modifier.padding(horizontal = 8.dp), Arrangement.spacedBy(8.dp)) {
                        if (kind == SCENES) WordAction(main::examples) { Text(stringResource(R.string.examples)) }
                        WordAction(main::openPackage) { Text(stringResource(R.string.open_package)) }
                    }
                    Importing(main)
                    if (query.isEmpty()) AwayNotes(main)
                    if (items?.isEmpty() == true) Empty(main, hollow(kind, searched), searched, reset, sources)
                }
            }
            items(shown, key = Item::uid) { item ->
                val tap = { if (chosen.isEmpty()) open(item.uid) else chosen.toggle(item.uid) }
                val press = Modifier.combinedClickable(onLongClick = { chosen.toggle(item.uid) }, onClick = tap)
                Cover(item, press, item.uid == inUse, item.uid in chosen)
            }
        }
    }
    val foot = Modifier.align(Alignment.BottomCenter).padding(start = Measure.e3, end = Measure.e3, bottom = Measure.e3)
    if (!chosen.isEmpty()) ChosenBar(main, chosen, null, foot) {
        made = it
        main.setWallpaper()
    }
    made?.let { Rotated(main, it, foot) { made = null } }
    if (adding) Adding(main, sources) { adding = false }
}

private fun Modifier.bleed(edge: Dp) = layout { measurable, constraints ->
    val side = edge.roundToPx()
    val wide = constraints.maxWidth + side * 2
    val placed = measurable.measure(constraints.copy(minWidth = wide, maxWidth = wide))
    layout(constraints.maxWidth, placed.height) { placed.place(-side, 0) }
}

private const val SCENES = "kind:scene"
private const val SEARCH_MS = 300L
