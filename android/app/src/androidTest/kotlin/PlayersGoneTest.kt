package io.github.wallpaper_rs

import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class PlayersGoneTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val painter = Painter(false, Videos(context), Tilt(context), Hearing(context), Pages(context))
    private val first = Players(context)
    private val second = Players(context)
    private val session = MediaSession(context, "wprs-gone-test")

    private fun allow(on: Boolean) {
        Device.listener(on)
        assertTrue(waitFor { Players.allowed(context) == on })
    }

    private fun state(kind: Int) = PlaybackState.Builder().setState(kind, 1_000, 1f).build()

    @After
    fun stop() {
        Render.call {
            first.close()
            second.close()
        }
        session.release()
        allow(false)
    }

    @Test
    fun aReaderLeavesTheSummaryWhenItsOwnerIsGone() {
        session.setMetadata(MediaMetadata.Builder().putString(MediaMetadata.METADATA_KEY_TITLE, "Tone").build())
        session.setPlaybackState(state(PlaybackState.STATE_PLAYING))
        session.isActive = true
        allow(true)
        Render.call { first.follow(painter, true) }
        assertTrue("first reads a playing track ${Players.readersSeen()}", waitFor { Players.status.let { it.track && it.playing } })
        Render.call { first.close() }
        assertFalse("a closed reader left a track ${Players.readersSeen()}", Players.status.track)
        session.setPlaybackState(state(PlaybackState.STATE_PAUSED))
        Render.call { second.follow(painter, true) }
        assertTrue("second follows a pause, summary ${Players.status} readers ${Players.readersSeen()}", waitFor { Players.status.let { it.track && !it.playing } })
    }
}
