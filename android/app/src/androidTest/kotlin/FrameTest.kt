package io.github.wallpaper_rs

import android.os.Debug
import org.junit.Assert.assertTrue
import org.junit.Assume.assumeTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class FrameTest : SceneBase() {
    @Test
    fun drawingFramesMakesNoNativeAllocationsBesidesTheGpuCalls() {
        show("Snowfall")
        assertTrue("no ${FRAMES} frames within the patience", waitFor(PATIENCE) { frames.get() > FRAMES })
        Render.call { Native.allocations(true) }
        val start = frames.get()
        val waited = waitFor(PATIENCE) { frames.get() > start + MEASURED }
        val made = Render.call { Native.allocations(false) }
        val drawn = frames.get() - start
        assertTrue("fewer than $MEASURED frames within the patience", waited)
        assertTrue("the render thread made $made native allocations in $drawn frames", made <= drawn)
    }

    @Suppress("DEPRECATION")
    @Test
    fun drawingFramesAllocatesNothingOnTheRenderThread() {
        show("Snowfall")
        assertTrue("no ${FRAMES} frames within the patience", waitFor(PATIENCE) { frames.get() > FRAMES })
        val counted = Render.call {
            Debug.startAllocCounting()
            Debug.resetThreadAllocSize()
            val probe = ByteArray(PROBE)
            Debug.getThreadAllocSize() + probe.size * 0
        }
        assumeTrue("this runtime does not count allocations", counted >= PROBE)
        try {
            Render.call { Debug.resetThreadAllocSize() }
            val start = frames.get()
            assertTrue("fewer than $MEASURED frames within the patience", waitFor(PATIENCE) { frames.get() > start + MEASURED })
            val bytes = Render.call { Debug.getThreadAllocSize() }
            assertTrue("the render thread allocated $bytes bytes in $MEASURED frames", bytes <= NOISE)
        } finally {
            Render.call { Debug.stopAllocCounting() }
        }
    }

    private companion object {
        const val PATIENCE = 120_000L
        const val PROBE = 4096
        const val MEASURED = 60
        const val NOISE = 4096
    }
}
