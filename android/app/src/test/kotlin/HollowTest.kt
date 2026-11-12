package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Test

class HollowTest {
    @Test
    fun everyFilterHasItsOwnEmptyState() {
        assertEquals(Hollow.LIBRARY, hollow("", ""))
        assertEquals(Hollow.IMAGES, hollow("kind:image", ""))
        assertEquals(Hollow.VIDEOS, hollow("kind:video", ""))
        assertEquals(Hollow.SCENES, hollow("kind:scene", ""))
        assertEquals(Hollow.WEBS, hollow("kind:web", ""))
        assertEquals(Hollow.FAVORITES, hollow("fav:", ""))
    }

    @Test
    fun aSearchHasItsOwnEmptyStateWhateverTheFilter() {
        assertEquals(Hollow.SEARCH, hollow("", "nieve"))
        assertEquals(Hollow.SEARCH, hollow("kind:web", "nieve"))
    }

    @Test
    fun theWebStateDoesNotTalkAboutScenes() {
        assertEquals(R.string.empty_webs, Hollow.WEBS.text)
    }
}
