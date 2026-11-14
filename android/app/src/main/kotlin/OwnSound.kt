package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import kotlin.math.roundToInt

@Composable
fun OwnSound(main: Main, item: Item) {
    var level by remember(item.uid, item.loudness) { mutableFloatStateOf(item.loudness / 100f) }
    Toggle(stringResource(R.string.wallpaper_sound), !item.silent) {
        main.act { ask("sound", "uids" to listOf(item.uid), "muted" to (!it).toString()) }
    }
    Rail(
        stringResource(R.string.wallpaper_volume), "${(level * 100).roundToInt()}%", level, 0f..1f, 19, !item.silent,
        change = { level = it },
    ) { main.act { ask("sound", "uids" to listOf(item.uid), "volume" to (it * 100).roundToInt().toString()) } }
}
