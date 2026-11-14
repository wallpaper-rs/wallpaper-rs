package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class PassageTest {
    @Test
    fun theRowSaysWhatReallyHappensOnThisPhone() {
        assertEquals(R.string.passage_own, Passage.note(Passage.OWN, still = false, saving = false, quality = null))
        assertEquals(R.string.passage_fade, Passage.note(Passage.FADE, still = false, saving = false, quality = null))
        assertEquals(R.string.passage_cut, Passage.note(Passage.NONE, still = true, saving = true, quality = 0))
        assertEquals("less motion cuts every change", R.string.passage_still, Passage.note(Passage.OWN, still = true, saving = false, quality = null))
        assertEquals(R.string.passage_still, Passage.note(Passage.FADE, still = true, saving = true, quality = null))
        assertEquals("a paused wallpaper cuts", R.string.passage_saving, Passage.note(Passage.OWN, still = false, saving = true, quality = 3))
        assertEquals("a fixed low quality plays only light ones", R.string.passage_light, Passage.note(Passage.OWN, still = false, saving = false, quality = 1))
        assertEquals(R.string.passage_own, Passage.note(Passage.OWN, still = false, saving = false, quality = 2))
        assertEquals(R.string.passage_fade, Passage.note(Passage.FADE, still = false, saving = false, quality = 0))
    }

    @Test
    fun lessMotionFollowsTheSystemUnlessThePersonFixedIt() {
        assertTrue(Passage.still("system", system = true))
        assertFalse(Passage.still("system", system = false))
        assertTrue(Passage.still("always", system = false))
        assertFalse(Passage.still("never", system = true))
    }
}
