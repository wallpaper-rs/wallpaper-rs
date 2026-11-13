package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlin.math.roundToInt

@Composable
fun OwnSound(main: Main, item: Item) {
    var level by remember(item.uid, item.loudness) { mutableFloatStateOf(item.loudness / 100f) }
    Toggle(stringResource(R.string.wallpaper_sound), !item.silent) {
        main.act { ask("sound", "uids" to listOf(item.uid), "muted" to (!it).toString()) }
    }
    Column(Modifier.padding(16.dp, 0.dp)) {
        Text(stringResource(R.string.wallpaper_volume, (level * 100).roundToInt()))
        Slider(level, { level = it }, enabled = !item.silent, steps = 19, onValueChangeFinished = {
            main.act { ask("sound", "uids" to listOf(item.uid), "volume" to (level * 100).roundToInt().toString()) }
        })
    }
}
