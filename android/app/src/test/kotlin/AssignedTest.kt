package io.github.wallpaper_rs

import android.app.WallpaperManager
import org.junit.Assert.assertEquals
import org.junit.Test

class AssignedTest {
    @Test
    fun theScreensTheSystemChoseAreTheOnesRecorded() {
        assertEquals(HOME, assigned(BOTH, WallpaperManager.FLAG_SYSTEM))
        assertEquals(LOCK, assigned(BOTH, WallpaperManager.FLAG_LOCK))
        assertEquals(HOME, assigned(LOCK, WallpaperManager.FLAG_SYSTEM))
    }

    @Test
    fun bothScreensKeepWhatThePersonAskedFor() {
        val both = WallpaperManager.FLAG_SYSTEM or WallpaperManager.FLAG_LOCK
        assertEquals(BOTH, assigned(BOTH, both))
        assertEquals(HOME, assigned(HOME, both))
    }
}
