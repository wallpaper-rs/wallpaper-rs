package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import kotlin.math.roundToInt

const val VOLUME = "playback.volume"

object Loudness {
    fun parsed(saved: String?) = saved?.toFloatOrNull()?.coerceIn(0f, 1f) ?: 0.5f

    fun saved(level: Float) = ((level.coerceIn(0f, 1f) * 100).roundToInt() / 100.0).toString()
}

@Composable
fun Loudness(saved: String?, set: (String) -> Unit) =
    Glide(stringResource(R.string.volume), Loudness.parsed(saved) * PERCENT, 0f..PERCENT) { set(Loudness.saved(it / PERCENT)) }

private const val PERCENT = 100f
