package io.github.wallpaper_rs

import android.graphics.Color
import android.util.Log
import androidx.test.platform.app.InstrumentationRegistry
import java.util.concurrent.CopyOnWriteArrayList
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class TransitionCostTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val seen = CopyOnWriteArrayList<Pair<Long, Int>>()
    private val readers = Readers("passage-cost")
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
                seen += image.timestamp to Color.rgb(byte(0), byte(1), byte(2))
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
    fun aTransitionLeavesOneWallpaperAliveWhenItEnds() {
        Passages.pick("Red")
        Render.call {
            Native.commit(BOTH)
            painter.attach(reader.surface)
            painter.resize(SIDE, SIDE, 0)
            painter.show(true)
        }
        assertTrue(waitFor { seen.isNotEmpty() && near(Passages.RED, seen.last().second, LOOSE) })
        Thread.sleep(SETTLE_MS)
        val before = memory()
        val start = seen.size
        Passages.pick("Mark")
        Render.call {
            Native.commit(BOTH)
            Wallpaper.reload(false)
            painter.refresh()
        }
        var peak = before
        val end = System.currentTimeMillis() + LIMIT_MS
        while (System.currentTimeMillis() < end && !(seen.size > start && near(Passages.BLUE, seen.last().second, LOOSE))) {
            peak = peak.zip(memory()) { a, b -> maxOf(a, b) }
        }
        Thread.sleep(SETTLE_MS * 3)
        val after = memory()
        val marked = seen.drop(start).filter { Passages.marked(it.second) }.map { it.first }
        val gaps = marked.zipWithNext { a, b -> (b - a) / 1e6 }
        Log.i(TAG, "side $SIDE pss kb before ${before[0]} peak ${peak[0]} after ${after[0]}; graphics kb before ${before[1]} peak ${peak[1]} after ${after[1]}")
        Log.i(TAG, "transition frames ${marked.size}, gap ms mean ${"%.1f".format(gaps.average())} max ${"%.1f".format(gaps.maxOrNull() ?: 0.0)}")
        assertTrue("the mark of the transition never showed", marked.isNotEmpty())
        assertTrue("the passage kept its memory: ${before[0]} kb then ${after[0]} kb", after[0] - before[0] < KEPT_KB)
    }

    private fun memory(): List<Long> {
        val text = Device.shell("dumpsys meminfo ${context.packageName}")
        val row = { name: String -> Regex("$name\\s+(\\d+)").find(text)?.groupValues?.get(1)?.toLong() ?: 0L }
        return listOf(row("TOTAL PSS:"), row("Graphics:"))
    }

    private companion object {
        const val TAG = "wprs-measure"
        const val SIDE = 1440
        const val LOOSE = 24
        const val SETTLE_MS = 500L
        const val LIMIT_MS = 20_000L
        const val KEPT_KB = 12 * 1024
    }
}
