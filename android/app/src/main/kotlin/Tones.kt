package io.github.wallpaper_rs

import androidx.compose.runtime.Immutable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.graphics.Color

@Immutable
data class Tones(
    val night: Color,
    val nightHigh: Color,
    val nightLine: Color,
    val mist: Color,
    val mistLow: Color,
    val moon: Color,
    val moonDeep: Color,
    val veil: Float,
    val frost: Color = FROST,
    val amber: Color = AMBER,
    val coral: Color = CORAL,
    val sprout: Color = SPROUT,
) {
    val deep: Color get() = night

    fun veiled(alpha: Float): Color = night.copy(alpha = alpha.coerceIn(0f, 1f))

    companion object {
        private val FROST = Color(0xFFF4EFE6)
        private val AMBER = Color(0xFFF0C26B)
        private val CORAL = Color(0xFFFF9D8A)
        private val SPROUT = Color(0xFF8FD6A8)

        val NONE = Tones(
            night = Color(0xFF0A1322),
            nightHigh = Color(0xFF121E33),
            nightLine = Color(0xFF2A3A57),
            mist = Color(0xFFB9C4D6),
            mistLow = Color(0xFF8F9DB4),
            moon = Color(0xFF9CC3F5),
            moonDeep = Color(0xFF6F9FDC),
            veil = 0.8f,
        )
    }
}

object Look {
    private val held = mutableStateOf(Tones.NONE)

    val tones: Tones get() = held.value

    fun replace(next: Tones) {
        held.value = next
    }

    fun reset() = replace(Tones.NONE)
}

val LocalTones = staticCompositionLocalOf { Tones.NONE }
