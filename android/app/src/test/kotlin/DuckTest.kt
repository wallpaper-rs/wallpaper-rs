package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class DuckTest {
    @Test
    fun onlySoundOfOtherAppsSilencesTheWallpaper() {
        assertFalse(Ducking.others(0, 0))
        assertFalse("our own video plays alone", Ducking.others(1, 1))
        assertFalse("a video and a page with sound", Ducking.others(2, 2))
        assertTrue(Ducking.others(1, 0))
        assertTrue(Ducking.others(3, 2))
        assertFalse("our player was not reported yet", Ducking.others(0, 1))
    }

    @Test
    fun aVideoAtNoVolumeDecodesNoSound() {
        assertFalse(Audible.muted(silent = false, sound = "false", loudness = 0.5f))
        assertTrue("a muted package or a volume of 0", Audible.muted(silent = false, sound = "false", loudness = 0f))
        assertTrue("video sound off", Audible.muted(silent = false, sound = "true", loudness = 1f))
        assertTrue("a clip of a scene without sound", Audible.muted(silent = true, sound = "false", loudness = 1f))
    }

    @Test
    fun theGainFadesInTenStepsAndStopsAtItsGoal() {
        var gain = 1f
        var steps = 0
        while (gain != 0f) {
            gain = Ducking.next(gain, 0f)
            steps++
        }
        assertEquals(10, steps)
        assertEquals(0f, Ducking.next(0.05f, 0f))
        assertEquals(1f, Ducking.next(0.95f, 1f))
        assertEquals(0.5f, Ducking.next(0.4f, 1f), 1e-6f)
    }

    @Test
    fun anOwnPlayerThatStopsStillCountsUntilItsStreamIsGone() {
        val owners = Owners(1000)
        val video = Any()
        owners.want(video, true, 0)
        assertEquals(1, owners.count(10))
        owners.want(video, false, 100)
        assertFalse(owners.any())
        assertFalse("our stream may still be reported", Ducking.others(1, owners.count(500)))
        assertEquals(0, owners.count(1100))
        owners.want(video, false, 1200)
        assertEquals("a player that was never heard does not count", 0, owners.count(1300))
        owners.want(video, true, 1400)
        owners.want(video, false, 1500)
        owners.want(video, true, 1600)
        assertEquals("playing again cancels the grace", 1, owners.count(3000))
    }
}
