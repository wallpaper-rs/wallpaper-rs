package io.github.wallpaper_rs

import android.app.NotificationManager
import android.graphics.Color
import androidx.test.platform.app.InstrumentationRegistry
import java.util.concurrent.CopyOnWriteArrayList
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class TransitionTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val seen = CopyOnWriteArrayList<Int>()
    private val readers = Readers("passage")
    private val reader = readers.make(SIDE)
    private val painter = Painter(false, Videos(context), Tilt(blind(context)), Hearing(context), Pages(context))

    @Before
    fun prepare() {
        Passages.install()
        readers.listen(reader) { source ->
            source.acquireLatestImage()?.use { image ->
                val plane = image.planes[0]
                val at = SIDE / 2 * plane.rowStride + SIDE / 2 * plane.pixelStride
                val byte = { i: Int -> plane.buffer.get(at + i).toInt() and 0xff }
                seen += Color.rgb(byte(0), byte(1), byte(2))
            }
        }
        Device.setting("playback.allow_flashing", "true")
        Device.setting("transition.less_motion", "never")
        Device.powerReset()
    }

    @After
    fun detach() {
        Render.call { painter.detach() }
        readers.close()
    }

    @Test
    fun aWallpaperComesInWithItsOwnTransition() {
        val drawn = change("Mark")
        assertTrue("the mark of its transition never showed: ${drawn.map(::hex)}", drawn.any(Passages::marked))
    }

    @Test
    fun thePlainFadeLeavesTheTransitionOfTheWallpaperOut() {
        Device.setting("transition.kind", "fade")
        val drawn = change("Mark")
        assertFalse(drawn.any(Passages::marked))
        assertTrue("no frame mixed the two wallpapers: ${drawn.map(::hex)}", drawn.any(Passages::mixed))
    }

    @Test
    fun noTransitionCutsStraightToTheNextWallpaper() {
        Device.setting("transition.kind", "none")
        val drawn = change("Mark")
        assertFalse(drawn.map(::hex).toString(), drawn.any { Passages.marked(it) || Passages.mixed(it) })
    }

    @Test
    fun lessMotionCutsEvenAWallpaperThatBringsItsTransition() {
        Device.setting("transition.less_motion", "always")
        val drawn = change("Mark")
        assertFalse(drawn.map(::hex).toString(), drawn.any { Passages.marked(it) || Passages.mixed(it) })
    }

    @Test
    fun aChangeDuringATransitionEndsOnTheLastChoice() {
        show("Red")
        assertTrue(waitFor { seen.isNotEmpty() && near(Passages.RED, seen.last(), LOOSE) })
        val start = seen.size
        pick("Long")
        assertTrue("the long transition did not start", waitFor { seen.drop(start).any(Passages::marked) })
        pick("Green")
        assertTrue("the last choice did not settle: ${seen.lastOrNull()?.let(::hex)}", waitFor { near(Passages.GREEN, seen.last(), LOOSE) })
        val after = seen.size
        Thread.sleep(SETTLE_MS)
        assertFalse("the first transition went on over the last choice", seen.drop(after).any(Passages::marked))
    }

    @Test
    fun anInvalidTransitionGivesThePlainFadeAndSaysNothing() {
        val drawn = change("Broken")
        assertFalse(drawn.any(Passages::marked))
        assertTrue("no frame mixed the two wallpapers: ${drawn.map(::hex)}", drawn.any(Passages::mixed))
        val notices = context.getSystemService(NotificationManager::class.java).activeNotifications
        assertTrue("the person was told about it", notices.isEmpty())
    }

    private fun change(title: String): List<Int> {
        show("Red")
        assertTrue("red never showed: ${seen.lastOrNull()?.let(::hex)}", waitFor { seen.isNotEmpty() && near(Passages.RED, seen.last(), LOOSE) })
        val start = seen.size
        pick(title)
        assertTrue("blue never settled: ${seen.lastOrNull()?.let(::hex)}", waitFor { seen.size > start && near(Passages.BLUE, seen.last(), LOOSE) })
        Thread.sleep(SETTLE_MS)
        return seen.drop(start)
    }

    private fun pick(title: String) {
        Passages.pick(title)
        Render.call {
            Native.commit(BOTH)
            Wallpaper.reload(false)
            painter.refresh()
        }
    }

    private fun show(title: String) {
        Passages.pick(title)
        Render.call {
            Native.commit(BOTH)
            painter.attach(reader.surface)
            painter.resize(SIDE, SIDE, 0)
            painter.show(true)
        }
    }

    private companion object {
        const val SIDE = 64
        const val LOOSE = 24
        const val SETTLE_MS = 400L
    }
}
