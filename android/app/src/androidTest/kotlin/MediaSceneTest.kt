package io.github.wallpaper_rs

import android.graphics.Bitmap
import android.graphics.Color
import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import java.io.File
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class MediaSceneTest : SceneBase() {
    private val players = Players(context)
    private val session = MediaSession(context, "wprs-scene-test")

    private fun allow(on: Boolean) {
        Device.listener(on)
        assertTrue(waitFor { Players.allowed(context) == on })
    }

    private fun state(kind: Int) = PlaybackState.Builder().setState(kind, 1_000, 1f).build()

    @After
    fun stop() {
        Render.call { players.follow(painter, false) }
        session.release()
        allow(false)
    }

    @Test
    fun theStateAndTheCoverColourOfARealPlayerChangeWhatAMediaSceneDrawsAndStopWithoutAccess() {
        val dir = File(storage(), "tinted/tinted").apply { mkdirs() }
        File(dir, "scene.json").writeText(MEDIA_SCENE)
        File(dir, "manifest.json").writeText(MEDIA_MANIFEST)
        assertTrue(Native.library(JSONObject().put("op", "examples").put("dir", dir.parent).toString()).startsWith("["))
        val uid = show("Tinted")
        runBlocking { ask("grant", "uid" to uid, "input" to "media", "on" to true) }
        Render.call { painter.refresh() }
        assertTrue(waitFor { frames.get() > 0 })
        val red = { lamp.get() shr 8 }
        val green = { lamp.get() and 0xff }
        val cover = Bitmap.createBitmap(64, 64, Bitmap.Config.ARGB_8888).apply { eraseColor(Color.rgb(220, 20, 20)) }
        session.setMetadata(MediaMetadata.Builder().putString(MediaMetadata.METADATA_KEY_TITLE, "Tone").putBitmap(MediaMetadata.METADATA_KEY_ALBUM_ART, cover).build())
        session.setPlaybackState(state(PlaybackState.STATE_PAUSED))
        session.isActive = true
        allow(true)
        Render.call { players.follow(painter, true) }
        assertTrue("a paused player, summary ${Players.status} readers ${Players.readersSeen()}", waitFor(70_000) { Players.status.let { it.track && it.cover && !it.playing } })
        Render.call { painter.refresh() }
        assertTrue("a paused track left the scene dark: ${lamp.get()}", waitFor { red() < 32 })
        session.setPlaybackState(state(PlaybackState.STATE_PLAYING))
        assertTrue("playing did not show the cover colour: ${lamp.get()}", waitFor { red() > 150 && green() < 90 })
        session.setPlaybackState(state(PlaybackState.STATE_PAUSED))
        assertTrue("a pause did not take the colour away: ${lamp.get()}", waitFor { red() < 32 })
        session.setPlaybackState(state(PlaybackState.STATE_PLAYING))
        assertTrue(waitFor { red() > 150 })
        allow(false)
        assertTrue("taking access back stops reading", waitFor { !Players.status.listening })
        assertTrue("the scene kept the colour after access went: ${lamp.get()}", waitFor { red() < 32 })
    }
}
