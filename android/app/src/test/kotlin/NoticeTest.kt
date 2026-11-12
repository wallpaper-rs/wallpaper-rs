package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class NoticeTest {
    @Test
    fun theNotificationIsAskedForOnceAndOnlyWhereTheSystemAsks() {
        assertEquals(Prompt.ASK, Notice.plan(enabled = false, asked = false, sdk = 33))
        assertEquals(Prompt.SILENT, Notice.plan(enabled = false, asked = true, sdk = 36))
        assertEquals(Prompt.SILENT, Notice.plan(enabled = false, asked = false, sdk = 32))
        assertEquals(Prompt.SHOW, Notice.plan(enabled = true, asked = false, sdk = 36))
        assertEquals(Prompt.SHOW, Notice.plan(enabled = true, asked = true, sdk = 33))
    }
}
