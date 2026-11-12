package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class NoticeTest {
    @Test
    fun theNotificationIsAskedForOnceAndOnlyWhereTheSystemAsks() {
        assertEquals(Ask.ASK, Notice.plan(enabled = false, asked = false, sdk = 33))
        assertEquals(Ask.SILENT, Notice.plan(enabled = false, asked = true, sdk = 36))
        assertEquals(Ask.SILENT, Notice.plan(enabled = false, asked = false, sdk = 32))
        assertEquals(Ask.SHOW, Notice.plan(enabled = true, asked = false, sdk = 36))
        assertEquals(Ask.SHOW, Notice.plan(enabled = true, asked = true, sdk = 33))
    }
}
