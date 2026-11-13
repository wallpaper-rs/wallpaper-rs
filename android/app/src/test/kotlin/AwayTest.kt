package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class AwayTest {
    @Test
    fun anItemWithoutATitleIsNamedByItsRecord() {
        assertEquals("Hand made", awayName("Hand made", "9f9f9f9f9f9f"))
        assertEquals("9f9f9f9f", awayName("", "9f9f9f9f9f9f"))
    }

    @Test
    fun theNoteNamesTheRepositoryOnlyWhenThereIsOne() {
        assertEquals(R.string.away_note, awayNote(""))
        assertEquals(R.string.away_note_repo, awayNote("studio"))
    }
}
