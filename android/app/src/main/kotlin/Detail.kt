package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clipToBounds
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.platform.LocalWindowInfo
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

@Composable
fun Detail(main: Main, uid: String, open: (String) -> Unit, close: () -> Unit) {
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    val item by loaded<Item?>(null, uid, main.revision.intValue) {
        (ask("item", "uid" to uid, "languages" to languages) as? JSONObject)?.let(::Item)
    }
    val barred = item?.flashing == true
    var ready by remember(uid) { mutableStateOf(false) }
    var adding by remember { mutableStateOf(false) }
    var removing by remember { mutableStateOf(false) }
    var more by remember(uid) { mutableStateOf(false) }
    LaunchedEffect(uid, item == null, barred) {
        if (item == null || barred) return@LaunchedEffect
        runCatching { ask("pick", "choice" to choice("item", uid)) }
        ready = true
    }
    DisposableEffect(uid) { onDispose { main.unpick(choice("item", uid)) } }
    val picture = item?.run { !video && !scene } == true
    val state by loaded(JSONObject(), uid, picture, main.revision.intValue, Depth.working.value, powered.intValue) {
        if (picture) depth("item", "uid" to uid) as JSONObject else JSONObject()
    }
    val form by if (item?.scene == true || item?.reel == true) sceneForm(main, uid) else remember { mutableStateOf(JSONObject()) }
    val inUse by loaded(false, uid, main.revision.intValue) { nowOn() == uid }
    val report = standing(inUse)
    val undecided by loaded(emptyList<String>(), uid, picture, main.revision.intValue) { if (picture) emptyList() else asked(uid) }
    val window = LocalWindowInfo.current.containerSize
    val wide = window.width > window.height
    val screen = remember(window) { main.wallpaperSize() }
    val still by item?.let { thumbnail(it) } ?: remember { mutableStateOf(null) }
    val preview = @Composable { modifier: Modifier ->
        Box(modifier.testTag("preview"), Alignment.Center) {
            val shape = if (picture) Modifier.aspectRatio(screen.width.coerceAtLeast(1) / screen.height.coerceAtLeast(1).toFloat(), true) else Modifier.fillMaxSize()
            when {
                ready && (item?.refused != true || item?.animated == null) ->
                    Live(main, main.revision.intValue, Depth.working.value, modifier = if (picture) shape.framing(main, uid, screen) else shape, still = still)
                item?.animated != null -> item?.animated?.let { Animated(it, Modifier.fillMaxSize()) }
                else -> still?.let { Image(it, null, shape.testTag("still"), contentScale = ContentScale.Crop) }
            }
        }
    }
    val page: @Composable ColumnScope.() -> Unit = {
        if (inUse) item?.let { HeavyNotice(main, uid, it.title, report) }
        if (wide) item?.let { Heading(it, inUse, barred) }
        if (inUse) Standing(report)
        if (inUse && heavy(report)) PlayAnyway(main)
        Asks(main, uid, undecided)
        if (picture) Framing(main, uid, state)
        Own(main, uid, item, picture, state, form, undecided)
        item?.let { shown ->
            Fold(stringResource(R.string.more_options), more, Modifier.testTag("more_wallpaper")) { more = !more }
            if (more) About(main, uid, shown, form, open)
        }
    }
    Column(Modifier.fillMaxSize()) {
        if (wide) {
            Header(item?.title.orEmpty(), close)
            Row(Modifier.weight(1f)) {
                preview(Modifier.weight(1f).fillMaxHeight().padding(horizontal = Measure.e4))
                Column(Modifier.weight(1f).verticalScroll(rememberScrollState()), content = page)
            }
        } else {
            val tall = with(LocalDensity.current) { (window.height * PREVIEW_SHARE).toDp() }
            Column(Modifier.weight(1f).verticalScroll(rememberScrollState())) {
                Box(Modifier.fillMaxWidth().height(tall).clipToBounds().testTag("page_cover")) {
                    preview(Modifier.fillMaxSize())
                    item?.let {
                        Box(Modifier.align(Alignment.BottomStart).fillMaxWidth().background(Brush.verticalGradient(listOf(Color.Transparent, tones.night)))) {
                            Heading(it, inUse, barred)
                        }
                    }
                    Square(close, Ph.ArrowLeft, stringResource(R.string.back), Modifier.padding(Measure.e2), boxed = true)
                }
                page()
            }
        }
        Foot(main, uid, item, ready && !barred, { adding = true }) { removing = true }
    }
    if (adding) AddTo({ adding = false }) { playlist -> main.act { ask("add", "playlist" to playlist, "items" to listOf(uid)) } }
    if (removing) Removing(1, { removing = false }) {
        main.act {
            ask("remove", "uids" to listOf(uid))
            close()
        }
    }
}

@Composable
private fun Heading(item: Item, inUse: Boolean, barred: Boolean) = Column(Modifier.padding(start = Measure.margin, end = Measure.margin, top = Measure.e3, bottom = Measure.e2)) {
    Text(item.title, style = Type.titleM)
    Row(Modifier.padding(top = Measure.e1), Arrangement.spacedBy(Measure.e2), Alignment.CenterVertically) {
        if (inUse) InUse()
        Text(facts(item), color = tones.mist, style = Type.foot)
    }
    if (barred) StateLine(stringResource(R.string.flashes_barred), Tone.Amber, Modifier.padding(top = Measure.e2))
}

@Composable
private fun Foot(main: Main, uid: String, item: Item?, usable: Boolean, add: () -> Unit, remove: () -> Unit) =
    Row(Modifier.fillMaxWidth().padding(horizontal = Measure.e3, vertical = Measure.e3), Arrangement.spacedBy(Measure.e1), Alignment.CenterVertically) {
        val favorite = item?.favorite == true
        Square({ main.act { ask("favorite", "uids" to listOf(uid), "favorite" to !favorite) } }, Ph.Heart, stringResource(R.string.favorite), lit = favorite)
        Square(add, Ph.Playlist, stringResource(R.string.add_to_playlist))
        Square(remove, Ph.Trash, stringResource(R.string.remove))
        Action(main::setWallpaper, Modifier.weight(1f), enabled = usable) {
            Glyph(Ph.Image, null, size = Measure.iconSquare)
            Text(stringResource(R.string.apply))
        }
    }

private const val PREVIEW_SHARE = 0.36f
