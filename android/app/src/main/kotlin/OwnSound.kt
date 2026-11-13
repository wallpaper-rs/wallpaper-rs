package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlin.math.roundToInt

private const val OWN_MUTED = "playback.muted.default"
private const val OWN_VOLUME = "playback.volume.default"

@Composable
fun OwnSound(main: Main) {
    var changes by remember { mutableIntStateOf(0) }
    val values by loaded(emptyMap<String, String>(), changes) {
        listOf(OWN_MUTED, OWN_VOLUME).associateWith { Native.setting(it, null) }
    }
    val set = { key: String, value: String ->
        main.act {
            withContext(Dispatchers.IO) { Native.setting(key, value) }
            changes++
        }
    }
    val silent = values[OWN_MUTED] == "true"
    var level by remember(values[OWN_VOLUME]) { mutableFloatStateOf(values[OWN_VOLUME]?.toFloatOrNull()?.coerceIn(0f, 1f) ?: 1f) }
    Toggle(stringResource(R.string.wallpaper_sound), !silent) { set(OWN_MUTED, if (it) "false" else "true") }
    Column(Modifier.padding(16.dp, 0.dp)) {
        Text(stringResource(R.string.wallpaper_volume, (level * 100).roundToInt()))
        Slider(level, { level = it }, enabled = !silent, steps = 19, onValueChangeFinished = { set(OWN_VOLUME, Loudness.saved(level)) })
    }
}
