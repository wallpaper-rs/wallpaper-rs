package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

enum class Hollow(val text: Int) {
    LIBRARY(R.string.empty),
    IMAGES(R.string.empty_images),
    VIDEOS(R.string.empty_videos),
    SCENES(R.string.empty_scenes),
    WEBS(R.string.empty_webs),
    FAVORITES(R.string.empty_favorites),
    SEARCH(R.string.empty_search),
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
fun Empty(main: Main, why: Hollow) = Column(Modifier.padding(16.dp), Arrangement.spacedBy(12.dp)) {
    Text(stringResource(why.text))
    if (why == Hollow.LIBRARY) Button(main::examples) { Text(stringResource(R.string.examples)) }
    if (why == Hollow.LIBRARY || why == Hollow.IMAGES || why == Hollow.VIDEOS) {
        val pick = stringResource(R.string.pick_media)
        if (why == Hollow.LIBRARY) OutlinedButton(main::choose) { Text(pick) } else Button(main::choose) { Text(pick) }
    }
}
