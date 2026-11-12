package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TintTest {
    @Test
    fun coloursKeepTheirOpacityOnlyWhenThePropertyAllowsIt() {
        assertEquals(listOf(255, 128, 0, 255), Hex.parse("#ff8000", alpha = false))
        assertEquals(listOf(255, 128, 0, 64), Hex.parse("#FF800040", alpha = true))
        assertNull("no opacity for a plain colour", Hex.parse("#ff800040", alpha = false))
        assertNull(Hex.parse("#ff80", alpha = true))
        assertNull(Hex.parse("#gg8000", alpha = false))
        assertEquals("#ff8000", Hex.format(listOf(255, 128, 0, 64), alpha = false))
        assertEquals("#ff800040", Hex.format(listOf(255, 128, 0, 64), alpha = true))
        assertEquals("#ff0000ff", Hex.format(listOf(300, -1, 0, 255), alpha = true))
    }

    @Test
    fun textStopsAtItsLength() {
        assertEquals("abc", Typing(max = 3).fit("abcdef"))
        assertEquals("abcdef", Typing().fit("abcdef"))
        assertEquals("a\nb", Typing(multiline = true, max = 3).fit("a\nbc"))
    }
}
