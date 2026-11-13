package io.github.wallpaper_rs

import androidx.compose.foundation.layout.size
import androidx.compose.material3.Icon
import androidx.compose.material3.LocalContentColor
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp

@Composable
fun Glyph(
    ph: Ph,
    name: String?,
    modifier: Modifier = Modifier,
    tint: Color = LocalContentColor.current,
    lit: Boolean = false,
    size: Dp = Measure.icon,
) = Icon(painterResource(if (lit) ph.lit else ph.rest), name, modifier.size(size), tint)
