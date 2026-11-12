package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class AutoUpdateTest {
    @Test
    fun theDailyCheckRunsForEitherSetting() {
        assertTrue(updatesWanted(null, null))
        assertTrue(updatesWanted("true", "false"))
        assertTrue(updatesWanted("false", "true"))
        assertFalse(updatesWanted("false", "false"))
        assertFalse(updatesWanted("false", null))
    }

    @Test
    fun aHeldUpdateNamesWhatItNowAsks() {
        assertNull(askNote(emptyList()))
        assertEquals("audio, rgb", askNote(listOf("audio", "rgb")))
    }
}
