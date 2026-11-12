package io.github.wallpaper_rs

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioManager
import android.media.AudioPlaybackConfiguration
import android.os.SystemClock
import java.util.Collections
import java.util.WeakHashMap
import kotlin.math.abs
import kotlin.math.sign

interface Ducked {
    fun ducked()
}

object Ducking {
    const val STEP = 0.1f
    const val STEP_MS = 50L

    fun others(active: Int, ours: Int) = active > ours

    fun next(gain: Float, target: Float) = if (abs(target - gain) <= STEP) target else gain + sign(target - gain) * STEP
}

class Owners(private val graceMs: Long) {
    private val owners = Owners(GRACE_MS)
    private val leaving = mutableMapOf<Any, Long>()

    fun want(who: Any, on: Boolean, now: Long) {
        if (on) {
            wanting += who
            leaving -= who
        } else if (wanting.remove(who)) {
            leaving[who] = now + graceMs
        }
    }

    fun count(now: Long): Int {
        leaving.values.removeAll { it <= now }
        return wanting.size + leaving.size
    }

    fun any() = wanting.isNotEmpty()
}

object Audible {
    fun muted(silent: Boolean, sound: String?, loudness: Float) = silent || sound != "false" || loudness <= 0f
}

object Duck : AudioManager.AudioPlaybackCallback() {
    @Volatile
    var gain = 1f
        private set
    private var audio: AudioManager? = null
    private var target = 1f
    private val owners = Owners(GRACE_MS)
    private val listeners = Collections.newSetFromMap(WeakHashMap<Ducked, Boolean>())
    private val ramp = Runnable { step() }
    private val settle = Runnable { audio?.let { onPlaybackConfigChanged(it.activePlaybackConfigurations) } }

    fun listen(owner: Ducked) = Render.post { listeners += owner }

    fun unlisten(owner: Ducked) = Render.post { listeners -= owner }

    fun want(context: Context, who: Any, on: Boolean) = Render.post {
        owners.want(who, on, SystemClock.uptimeMillis())
        Render.cancel(settle)
        if (!on) Render.later(GRACE_MS + 1, settle)
        val listening = owners.any() && Native.setting(KEY, null) == "true"
        val manager = context.applicationContext.getSystemService(AudioManager::class.java) ?: return@post
        if (listening && audio == null) {
            audio = manager
            manager.registerAudioPlaybackCallback(this, Render.handler)
            onPlaybackConfigChanged(manager.activePlaybackConfigurations)
        } else if (!listening && audio != null) {
            audio?.unregisterAudioPlaybackCallback(this)
            audio = null
            aim(1f)
        }
    }

    override fun onPlaybackConfigChanged(configs: List<AudioPlaybackConfiguration>) =
        aim(if (Ducking.others(configs.count { it.audioAttributes.usage in PLAYERS }, owners.count(SystemClock.uptimeMillis()))) 0f else 1f)

    private fun aim(goal: Float) {
        target = goal
        Render.cancel(ramp)
        step()
    }

    private fun step() {
        if (gain == target) return
        gain = Ducking.next(gain, target)
        Native.duck(gain)
        listeners.toList().forEach(Ducked::ducked)
        if (gain != target) Render.later(Ducking.STEP_MS, ramp)
    }

    private const val KEY = "playback.duck"
    private const val GRACE_MS = 1000L
    private val PLAYERS = setOf(AudioAttributes.USAGE_MEDIA, AudioAttributes.USAGE_GAME, AudioAttributes.USAGE_UNKNOWN)
}
