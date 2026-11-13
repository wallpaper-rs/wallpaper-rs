package io.github.wallpaper_rs

import android.content.ComponentName
import android.content.Context
import android.database.ContentObserver
import android.graphics.Bitmap
import android.media.MediaMetadata
import android.media.session.MediaController
import android.media.session.MediaSessionManager
import android.media.session.PlaybackState
import android.os.SystemClock
import android.provider.Settings
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import java.util.Collections
import java.util.WeakHashMap
import org.json.JSONObject

class Pictured {
    private var sent = false

    fun read(fresh: Boolean): Boolean = (fresh || !sent).also { sent = true }

    fun forget() {
        sent = false
    }
}

object Retrying {
    private const val FIRST_MS = 1_000L
    private const val TRIES = 6

    fun after(failures: Int): Long? = if (failures < TRIES) FIRST_MS shl failures else null
}

class Players(private val context: Context) : MediaSessionManager.OnActiveSessionsChangedListener {
    private val listeners = mutableSetOf<Painter>()
    private val component = ComponentName(context, Listener::class.java)
    private var manager: MediaSessionManager? = null
    private var controller: MediaController? = null
    private val pictured = Pictured()
    private var count = 0
    private var covered = false
    private val callback = object : MediaController.Callback() {
        override fun onMetadataChanged(metadata: MediaMetadata?) = send(true)
        override fun onPlaybackStateChanged(state: PlaybackState?) = send(false)
        override fun onSessionDestroyed() = pick(null)
    }

    private val access = object : ContentObserver(Render.handler) {
        override fun onChange(selfChange: Boolean) = check()
    }
    private var observing = false

    private var failures = 0
    private val retry = Runnable { check() }

    init {
        synchronized(all) { all += this }
    }

    fun follow(painter: Painter, wanted: Boolean) {
        if (wanted) listeners += painter else listeners -= painter
        observe(listeners.isNotEmpty())
        check()
    }

    fun seen(): String = "allowed=${allowed(context)} following=${listeners.size} reading=${manager != null} failures=$failures players=$count track=${controller?.metadata != null}"

    fun close() {
        listeners.clear()
        observe(false)
        check()
        synchronized(all) { all -= this }
    }

    private fun observe(on: Boolean) {
        if (on == observing) return
        observing = on
        val resolver = context.contentResolver
        if (on) resolver.registerContentObserver(Settings.Secure.getUriFor(ENABLED), false, access) else resolver.unregisterContentObserver(access)
    }

    private fun check() {
        Render.cancel(retry)
        val on = listeners.isNotEmpty() && allowed(context)
        if (on == (manager != null)) return
        if (!on) {
            failures = 0
            manager?.removeOnActiveSessionsChangedListener(this)
            manager = null
            pick(null)
            return tell(MediaShown())
        }
        val sessions = context.getSystemService(MediaSessionManager::class.java) ?: return
        runCatching {
            sessions.addOnActiveSessionsChangedListener(this, component, Render.handler)
            manager = sessions
            failures = 0
            onActiveSessionsChanged(sessions.getActiveSessions(component))
        }.onFailure {
            Native.media(null, null, 0, false)
            tell(MediaShown())
            Retrying.after(failures++)?.let { Render.later(it, retry) }
        }
    }

    override fun onActiveSessionsChanged(controllers: List<MediaController>?) {
        count = controllers?.size ?: 0
        val playing = controllers?.firstOrNull { it.playbackState?.state == PlaybackState.STATE_PLAYING }
        pick(playing ?: controllers?.firstOrNull())
    }

    private fun pick(next: MediaController?) {
        if (next?.sessionToken == controller?.sessionToken && next != null) return send(false)
        controller?.unregisterCallback(callback)
        controller = next
        pictured.forget()
        next?.registerCallback(callback, Render.handler)
        send(true)
    }

    private fun send(fresh: Boolean) {
        val chosen = controller
        val metadata = chosen?.metadata
        if (chosen == null || metadata == null || manager == null) {
            pictured.forget()
            covered = false
            val reading = manager != null && allowed(context)
            Native.media(null, null, 0, reading)
            tell(MediaShown(reading, if (reading) count else 0))
        } else {
            val state = chosen.playbackState
            val same = !pictured.read(fresh)
            val picture = if (same) null else metadata.getBitmap(MediaMetadata.METADATA_KEY_ALBUM_ART) ?: metadata.getBitmap(MediaMetadata.METADATA_KEY_ART)
            if (!same) covered = picture != null
            val text = { key: String -> metadata.getString(key).orEmpty() }
            val told = JSONObject()
                .put("title", text(MediaMetadata.METADATA_KEY_TITLE))
                .put("artist", text(MediaMetadata.METADATA_KEY_ARTIST))
                .put("album", text(MediaMetadata.METADATA_KEY_ALBUM))
                .put("album_artist", text(MediaMetadata.METADATA_KEY_ALBUM_ARTIST))
                .put("genres", text(MediaMetadata.METADATA_KEY_GENRE))
                .put("playing", state?.state == PlaybackState.STATE_PLAYING)
                .put("position", position(state) / 1000.0)
                .put("duration", metadata.getLong(MediaMetadata.METADATA_KEY_DURATION).coerceAtLeast(0) / 1000.0)
                .put("rate", state?.playbackSpeed?.toDouble() ?: 1.0)
                .put("keep_cover", same)
            val small = picture?.let(::shrink)
            val pixels = small?.let { IntArray(it.width * it.height).also { all -> it.getPixels(all, 0, it.width, 0, 0, it.width, it.height) } }
            Native.media(told.toString(), pixels, small?.width ?: 0, true)
            tell(MediaShown(true, count, true, told.getBoolean("playing"), covered))
        }
        listeners.forEach(Painter::move)
    }

    private fun tell(shown: MediaShown) = synchronized(readers) {
        if (shown.listening) readers[this] = shown else readers.remove(this)
        status = MediaShown.of(readers.values)
    }

    private fun position(state: PlaybackState?): Long {
        state ?: return 0
        if (state.state != PlaybackState.STATE_PLAYING) return state.position.coerceAtLeast(0)
        val spent = SystemClock.elapsedRealtime() - state.lastPositionUpdateTime
        return (state.position + spent * state.playbackSpeed).toLong().coerceAtLeast(0)
    }

    private fun shrink(picture: Bitmap): Bitmap {
        val side = maxOf(picture.width, picture.height)
        if (side <= SIDE) return picture.copy(Bitmap.Config.ARGB_8888, false)
        val (w, h) = maxOf(1, picture.width * SIDE / side) to maxOf(1, picture.height * SIDE / side)
        return Bitmap.createScaledBitmap(picture.copy(Bitmap.Config.ARGB_8888, false), w, h, true)
    }

    companion object {
        private const val SIDE = 256
        private const val ENABLED = "enabled_notification_listeners"
        private val readers = WeakHashMap<Players, MediaShown>()
        private val all = Collections.newSetFromMap(WeakHashMap<Players, Boolean>())
        var status by mutableStateOf(MediaShown())
            private set
        var changes by mutableIntStateOf(0)
            private set

        fun readersSeen(): String = synchronized(readers) { readers.values.toString() }

        fun recheck() = Render.post {
            changes++
            synchronized(all) { all.toList() }.forEach(Players::check)
        }

        fun allowed(context: Context): Boolean {
            val enabled = Settings.Secure.getString(context.contentResolver, ENABLED).orEmpty()
            return enabled.split(':').any { ComponentName.unflattenFromString(it) == ComponentName(context, Listener::class.java) }
        }
    }
}
