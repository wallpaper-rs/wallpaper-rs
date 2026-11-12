package io.github.wallpaper_rs

import android.content.ComponentName
import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.os.ParcelFileDescriptor
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class PlayersTest {
    private val instrumentation = InstrumentationRegistry.getInstrumentation()
    private val context = instrumentation.targetContext
    private val listener = ComponentName(context, Listener::class.java).flattenToString()
    private val painter = Painter(false, Videos(context), Tilt(context), Hearing(context), Pages(context))
    private val players = Players(context)
    private val session = MediaSession(context, "wprs-test")

    private fun shell(command: String) = ParcelFileDescriptor.AutoCloseInputStream(instrumentation.uiAutomation.executeShellCommand(command)).use { it.readBytes() }

    private fun allow(on: Boolean) {
        shell("cmd notification ${if (on) "allow_listener" else "disallow_listener"} $listener")
        assertTrue(waitFor { Players.allowed(context) == on })
    }

    @After
    fun stop() {
        Render.call { players.follow(painter, false) }
        session.release()
        allow(false)
    }

    @Test
    fun aRealPlayerIsReadAndTheListenerComesBackAfterAccessReturns() {
        session.setMetadata(MediaMetadata.Builder().putString(MediaMetadata.METADATA_KEY_TITLE, "Tone").putLong(MediaMetadata.METADATA_KEY_DURATION, 60_000).build())
        session.setPlaybackState(PlaybackState.Builder().setState(PlaybackState.STATE_PLAYING, 1_000, 1f).build())
        session.isActive = true
        allow(true)
        Render.call { players.follow(painter, true) }
        assertTrue("the player is read", waitFor { Players.status.let { it.listening && it.track && it.playing && it.players > 0 } })
        session.setPlaybackState(PlaybackState.Builder().setState(PlaybackState.STATE_PAUSED, 2_000, 1f).build())
        assertTrue("a pause is seen", waitFor { Players.status.let { it.track && !it.playing } })
        allow(false)
        assertTrue("taking access back stops reading", waitFor { !Players.status.listening })
        assertFalse(Players.status.track)
        allow(true)
        assertTrue("the listener starts again by itself", waitFor { Players.status.let { it.listening && it.track } })
    }
}
