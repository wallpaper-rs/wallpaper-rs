package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class LoudnessTest {
    @Test
    fun theVolumeIsReadAndSavedBetweenZeroAndOne() {
        assertEquals(0.5f, Loudness.parsed(null))
        assertEquals(0.5f, Loudness.parsed("loud"))
        assertEquals(0.2f, Loudness.parsed("0.2"))
        assertEquals(1f, Loudness.parsed("3"))
        assertEquals("0.35", Loudness.saved(0.3499f))
        assertEquals("0.0", Loudness.saved(-1f))
        assertEquals("1.0", Loudness.saved(2f))
    }
}
