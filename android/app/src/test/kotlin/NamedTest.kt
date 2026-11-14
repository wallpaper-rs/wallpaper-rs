package io.github.wallpaper_rs

import java.time.LocalDate
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class NamedTest {
    private val words = { date: LocalDate? -> date?.let { "Photo from $it" } ?: "Photo" }

    @Test
    fun systemNumbersAndCameraCodesAreCoded() {
        assertTrue(coded("1000000032"))
        assertTrue(coded("20261114_110214"))
        assertTrue(coded("a1_b2-c3"))
        assertFalse(coded("Dunas"))
        assertFalse(coded("IMG_20261114_110214"))
        assertFalse(coded("Dunas al sol"))
        assertFalse(coded(""))
    }

    @Test
    fun theDateComesFromTheFileOrItsName() {
        assertEquals(LocalDate.of(2026, 11, 14), stamped("20261114_110214"))
        assertNull(stamped("1000000032"))
        assertNull(stamped("20261399"))
    }

    @Test
    fun onlyCodedNamesAreReplacedAndTheEndingStays() {
        assertEquals("Photo.jpg", readable("1000000032.jpg", null, words))
        assertEquals("Photo from 2026-11-14.jpg", readable("20261114_110214.jpg", null, words))
        assertEquals("Photo from 2026-11-14.png", readable("1000000032.png", LocalDate.of(2026, 11, 14).atTime(12, 0).atZone(java.time.ZoneId.systemDefault()).toInstant().toEpochMilli(), words))
        assertEquals("Dunas.jpg", readable("Dunas.jpg", 1L, words))
        assertEquals("IMG_20261114.jpg", readable("IMG_20261114.jpg", null, words))
    }
}
