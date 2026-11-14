package io.github.wallpaper_rs

import androidx.compose.ui.graphics.toArgb
import androidx.core.graphics.ColorUtils
import org.json.JSONObject
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class ShadeTest {
    private fun shaded(argb: Int) = tonesOf(JSONObject(Native.tones(IntArray(96 * 54) { argb }, 96)))

    private fun contrast(text: Int, ground: Int) = ColorUtils.calculateContrast(text, ground)

    @Test
    fun textKeepsItsContrastOnBrightAndDarkWallpapers() {
        val bright = shaded(0xFFF8F4E8.toInt())
        val dark = shaded(0xFF05060A.toInt())
        listOf(bright, dark).forEach { tones ->
            assertTrue(contrast(tones.frost.toArgb(), tones.night.toArgb()) >= 7.0)
            assertTrue(contrast(tones.mist.toArgb(), tones.night.toArgb()) >= 4.5)
        }
        assertNotEquals(shaded(0xFF40A0F0.toInt()).night, shaded(0xFF601010.toInt()).night)
    }
}
