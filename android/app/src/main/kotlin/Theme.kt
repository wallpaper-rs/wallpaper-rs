package io.github.wallpaper_rs

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Shapes
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.ReadOnlyComposable
import androidx.compose.ui.unit.dp

object Measure {
    val e1 = 4.dp
    val e2 = 8.dp
    val e3 = 12.dp
    val e4 = 16.dp
    val e5 = 20.dp
    val e6 = 24.dp
    val e8 = 32.dp
    val e10 = 40.dp
    val margin = 20.dp
    val touch = 48.dp
    val bar = 64.dp
    val row = 64.dp
    val option = 56.dp
    val field = 52.dp
    val rControl = 6.dp
    val rNotice = 10.dp
    val rSheet = 22.dp
    val line = 1.dp
    val icon = 24.dp
    val iconSmall = 19.dp
    val iconSquare = 20.dp
    val iconFold = 17.dp
    val buttonStart = 18.dp
    val mark = 24.dp
    val markHeight = 4.dp
    val handle = 36.dp
    val rail = 3.dp
    val switchWidth = 52.dp
    val switchHeight = 32.dp
    val knobOff = 20.dp
    val knobOn = 22.dp
}

val tones: Tones
    @Composable @ReadOnlyComposable get() = LocalTones.current

fun Tones.scheme() = darkColorScheme(
    primary = moon,
    onPrimary = night,
    primaryContainer = nightLine,
    onPrimaryContainer = frost,
    inversePrimary = moonDeep,
    secondary = mist,
    onSecondary = night,
    secondaryContainer = nightLine,
    onSecondaryContainer = frost,
    tertiary = sprout,
    onTertiary = night,
    tertiaryContainer = nightLine,
    onTertiaryContainer = frost,
    background = night,
    onBackground = frost,
    surface = night,
    onSurface = frost,
    surfaceVariant = nightHigh,
    onSurfaceVariant = mist,
    surfaceTint = night,
    inverseSurface = frost,
    inverseOnSurface = night,
    error = coral,
    onError = night,
    errorContainer = nightLine,
    onErrorContainer = coral,
    outline = nightLine,
    outlineVariant = nightLine,
    scrim = night,
    surfaceBright = nightLine,
    surfaceDim = night,
    surfaceContainerLowest = night,
    surfaceContainerLow = nightHigh,
    surfaceContainer = nightHigh,
    surfaceContainerHigh = nightHigh,
    surfaceContainerHighest = nightLine,
)

private val SHAPES = Shapes(
    extraSmall = RoundedCornerShape(Measure.rControl),
    small = RoundedCornerShape(Measure.rControl),
    medium = RoundedCornerShape(Measure.rNotice),
    large = RoundedCornerShape(Measure.rNotice),
    extraLarge = RoundedCornerShape(Measure.rSheet),
)

@Composable
fun Theme(content: @Composable () -> Unit) {
    val now = Look.tones
    CompositionLocalProvider(LocalTones provides now) {
        MaterialTheme(colorScheme = now.scheme(), typography = Type.material, shapes = SHAPES) {
            CompositionLocalProvider(LocalContentColor provides now.frost, content = content)
        }
    }
}
