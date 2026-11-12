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

const val VOLUME = "playback.volume"

object Loudness {
    fun parsed(saved: String?) = saved?.toFloatOrNull()?.coerceIn(0f, 1f) ?: 0.5f

    fun saved(level: Float) = ((level.coerceIn(0f, 1f) * 100).roundToInt() / 100.0).toString()
}

@Composable
fun Loudness(saved: String?, enabled: Boolean, set: (String) -> Unit) {
    var level by remember(saved) { mutableFloatStateOf(Loudness.parsed(saved)) }
    Column(Modifier.padding(16.dp, 0.dp)) {
        Text(stringResource(R.string.sound_volume, (level * 100).roundToInt()))
        Slider(level, { level = it }, enabled = enabled, steps = 19, onValueChangeFinished = { set(Loudness.saved(level)) })
    }
}
