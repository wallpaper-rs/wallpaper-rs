package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource

enum class Hollow(val text: Int) {
    LIBRARY(R.string.empty),
    IMAGES(R.string.empty_images),
    VIDEOS(R.string.empty_videos),
    SCENES(R.string.empty_scenes),
    WEBS(R.string.empty_webs),
    FAVORITES(R.string.lib_no_favorites),
    SEARCH(R.string.lib_nothing_with),
}

fun hollow(kind: String, searched: String): Hollow = when {
    searched.isNotEmpty() -> Hollow.SEARCH
    kind == "kind:image" -> Hollow.IMAGES
    kind == "kind:video" -> Hollow.VIDEOS
    kind == "kind:scene" -> Hollow.SCENES
    kind == "kind:web" -> Hollow.WEBS
    kind == "fav:" -> Hollow.FAVORITES
    else -> Hollow.LIBRARY
}

@Composable
fun Empty(main: Main, why: Hollow, searched: String = "", reset: () -> Unit = {}, sources: () -> Unit) = when (why) {
    Hollow.SEARCH -> Blank(stringResource(why.text, searched), stringResource(R.string.lib_nothing_with_note), Modifier.padding(top = Measure.e8).testTag("empty")) {
        QuietAction(reset, Modifier.fillMaxWidth()) {
            Glyph(Ph.X, null, Modifier.padding(end = Measure.e2), tones.frost, size = Measure.iconSmall)
            Text(stringResource(R.string.clear_search))
        }
    }
    Hollow.FAVORITES -> Blank(stringResource(why.text), stringResource(R.string.lib_no_favorites_note), Modifier.padding(top = Measure.e8).testTag("empty")) {
        QuietAction(reset, Modifier.fillMaxWidth()) {
            Glyph(Ph.Images, null, Modifier.padding(end = Measure.e2), tones.frost, size = Measure.iconSmall)
            Text(stringResource(R.string.lib_see_all))
        }
    }
    else -> Others(main, why, sources)
}

@Composable
private fun Others(main: Main, why: Hollow, sources: () -> Unit) = when (why) {
    Hollow.LIBRARY -> Column(Modifier.padding(top = Measure.e8).testTag("empty")) {
        Blank(stringResource(R.string.empty_title), stringResource(why.text)) {
            Action(sources, Modifier.fillMaxWidth().testTag("explore")) {
                Glyph(Ph.Compass, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
                Text(stringResource(R.string.explore_repos))
            }
            QuietAction(main::choose, Modifier.fillMaxWidth()) {
                Glyph(Ph.Image, null, Modifier.padding(end = Measure.e2), tones.frost, size = Measure.iconSmall)
                Text(stringResource(R.string.add_photos))
            }
        }
        Bundled(main)
    }
    else -> Column(Modifier.padding(horizontal = Measure.margin, vertical = Measure.e4), Arrangement.spacedBy(Measure.e3)) {
        Text(stringResource(why.text))
        if (why == Hollow.IMAGES || why == Hollow.VIDEOS) Action(main::choose) { Text(stringResource(R.string.pick_media)) }
        QuietAction(sources) { Text(stringResource(R.string.explore_repos)) }
    }
}
