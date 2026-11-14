package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay
import androidx.compose.runtime.remember
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.IconButton
import androidx.compose.foundation.layout.fillMaxWidth

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
            main.search.value = null
        }
    }
    val chosen = rememberSelection()
    val kind = FILTERS[filter].second
    val query = listOf(kind, searched).filter(String::isNotEmpty).joinToString(" ")
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    val items by loaded<List<Item>?>(null, main.revision.intValue, query, languages) {
        fetch("items", "query" to query, "languages" to languages)
    }
    chosen.follow(items)
    BackHandler(text.isNotEmpty() || filter != 0) {
        text = ""
        searched = ""
        filter = 0
    }
    Column {
        if (chosen.isEmpty()) Header(stringResource(R.string.library)) else Selecting(main, chosen, items.orEmpty().map(Item::uid))
        OutlinedTextField(
            text, { text = it }, Modifier.fillMaxWidth().padding(horizontal = 16.dp), singleLine = true,
            placeholder = { Text(stringResource(R.string.search)) }, leadingIcon = { Glyph(Ph.MagnifyingGlass, null) },
            trailingIcon = { if (text.isNotEmpty()) IconButton({ text = "" }) { Glyph(Ph.X, stringResource(R.string.clear_search)) } },
        )
        Choices(null, FILTERS.map { (label, query) -> query to stringResource(label) }, kind) { chosen ->
            filter = FILTERS.indexOfFirst { it.second == chosen }
        }
        if (kind == SCENES || kind == "kind:web") Row(Modifier.padding(horizontal = 8.dp), Arrangement.spacedBy(8.dp)) {
            if (kind == SCENES) WordAction(main::examples) { Text(stringResource(R.string.examples)) }
            WordAction(main::openPackage) { Text(stringResource(R.string.open_package)) }
        }
        Importing(main)
        if (query.isEmpty()) AwayNotes(main)
        if (items?.isEmpty() == true) Empty(main, hollow(kind, searched), sources)
        val shown = items.orEmpty()
        LazyVerticalGrid(
            columns = GridCells.Adaptive(112.dp),
            contentPadding = PaddingValues(16.dp, 8.dp, 16.dp, 96.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            items(shown, key = Item::uid) { item ->
                val tap = { if (chosen.isEmpty()) open(item.uid) else chosen.toggle(item.uid) }
                val press = Modifier.combinedClickable(onLongClick = { chosen.toggle(item.uid) }, onClick = tap)
                Thumb(item, Modifier.aspectRatio(0.75f).then(press), item.uid in chosen)
            }
        }
    }
    if (chosen.isEmpty()) Adding(main, sources, Modifier.align(Alignment.BottomEnd).padding(16.dp))
}

@Composable
fun Thumb(item: Item, modifier: Modifier = Modifier, picked: Boolean = false) {
    val bitmap by thumbnail(item)
    val shape = MaterialTheme.shapes.medium
    val outline = if (picked) Modifier.border(3.dp, MaterialTheme.colorScheme.primary, shape) else Modifier
    Box(modifier.then(outline).clip(shape).background(MaterialTheme.colorScheme.surfaceVariant)) {
        val kind = stringResource(if (item.video) R.string.kind_video else if (item.scene) R.string.kind_scene else R.string.kind_photo)
        bitmap?.let { Image(it, "$kind ${item.title}".trim(), Modifier.fillMaxSize(), contentScale = ContentScale.Crop) }
        if (item.flashing) StateLine(stringResource(R.string.flashes_barred), Tone.Amber, Modifier.align(Alignment.TopStart).fillMaxWidth().background(tones.nightHigh).padding(Measure.e1))
        if (item.failing != null) Glyph(Ph.Warning, stringResource(R.string.page_error), Modifier.align(Alignment.BottomEnd).padding(4.dp), MaterialTheme.colorScheme.error)
        val corner = Modifier.align(Alignment.BottomStart).padding(4.dp)
        if (item.video) Glyph(Ph.Play, null, corner, tones.frost)
        if (item.scene) Glyph(if (item.web) Ph.GlobeSimple else Ph.Sparkle, null, corner, tones.frost)
        if (picked) Glyph(Ph.CheckCircle, null, Modifier.align(Alignment.TopStart).padding(4.dp), MaterialTheme.colorScheme.primary)
    }
}

private const val SCENES = "kind:scene"
private const val SEARCH_MS = 300L
