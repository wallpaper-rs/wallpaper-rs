package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class RetryingTest {
    @Test
    fun theMediaListenerTriesAgainAFewTimesWaitingLongerEachTime() {
        assertEquals(listOf(1_000L, 2_000L, 4_000L, 8_000L, 16_000L, 32_000L, null), List(7) { Retrying.after(it) })
    }
}
