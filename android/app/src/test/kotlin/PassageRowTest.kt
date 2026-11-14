package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class PassageRowTest {
    private fun note(vararg pairs: Pair<String, String>, system: Boolean = false) = passageNote(mapOf(*pairs), system)

    @Test
    fun theTransitionRowExplainsTheStoredChoice() {
        assertEquals("an unset choice plays the wallpaper's own", R.string.passage_own, note())
        assertEquals(R.string.passage_cut, note("transition.kind" to "none"))
        assertEquals(R.string.passage_fade, note("transition.kind" to "fade"))
        assertEquals(R.string.passage_own, note("transition.kind" to "own", "playback.quality" to "auto"))
        assertEquals(R.string.passage_own, note("playback.quality" to "min-q1"))
    }

    @Test
    fun aFixedLowQualityAndLessMotionChangeTheWords() {
        assertEquals(R.string.passage_light, note("playback.quality" to "q1"))
        assertEquals(R.string.passage_light, note("playback.quality" to "q0"))
        assertEquals(R.string.passage_own, note("playback.quality" to "q2"))
        assertEquals(R.string.passage_still, note(LESS_MOTION to "always"))
        assertEquals("auto follows the system's animations", R.string.passage_still, note(LESS_MOTION to "auto", system = true))
        assertEquals(R.string.passage_own, note(LESS_MOTION to "never", system = true))
    }
}
