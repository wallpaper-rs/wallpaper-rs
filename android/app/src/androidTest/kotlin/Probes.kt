package io.github.wallpaper_rs

import android.app.NotificationManager
import android.app.UiModeManager
import android.content.ComponentName
import android.content.Context
import android.os.PowerManager
import android.provider.Settings
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.runner.lifecycle.ActivityLifecycleMonitorRegistry
import androidx.test.runner.lifecycle.Stage
import java.io.File

class Probe<T>(val name: String, private val read: () -> T, private val restore: (T) -> Unit) {
    fun capture() = Seen(this, read())

    fun current() = read()

    fun put(value: T) = restore(value)
}

class Seen<T>(private val probe: Probe<T>, private val value: T) {
    val name get() = probe.name

    fun same() = probe.current() == value

    fun now() = probe.current()

    fun restore() = probe.put(value)

    fun was() = value

    fun rebased() = Seen(probe, probe.current())
}

object Probes {
    const val BATTERY = "battery override"
    const val SAVER = "battery saver"
    const val NIGHT = "night mode"
    const val PREFERENCES = "preferences"
    const val NOTICES = "notifications allowed"
    const val LISTENER = "notification listener"
    val ANIMATIONS = listOf("window_animation_scale", "transition_animation_scale", "animator_duration_scale")
    val LISTENER_NAME get() = ComponentName(context, Listener::class.java).flattenToString()

    private val KEYS = listOf(
        "transition.kind", "fit", "playback.quality", "playback.flash_guard", "playback.duck", "playback.muted",
        "sync_network", "sync_listen", "video_convert", "repos.auto_refresh",
    )
    private val FOREIGN = listOf("Aw", "WebView", "com.google")
    private val context get() = InstrumentationRegistry.getInstrumentation().targetContext
    private val shell = Device::shell

    private fun global(name: String) = Settings.Global.getString(context.contentResolver, name).orEmpty()

    private fun filter(manager: NotificationManager) = manager.currentInterruptionFilter

    private fun live(): List<String> {
        var found = emptyList<String>()
        InstrumentationRegistry.getInstrumentation().runOnMainSync {
            found = Stage.values().filter { it != Stage.DESTROYED }.flatMap { stage ->
                ActivityLifecycleMonitorRegistry.getInstance().getActivitiesInStage(stage).map { "${it.javaClass.simpleName} $stage" }
            }
        }
        return found
    }

    fun noticesAllowed() = context.checkSelfPermission(android.Manifest.permission.POST_NOTIFICATIONS) == android.content.pm.PackageManager.PERMISSION_GRANTED

    private fun foreign(name: String) = FOREIGN.any(name::startsWith)

    private fun prefs(): Map<String, Map<String, Any?>> {
        val dir = File(context.applicationInfo.dataDir, "shared_prefs")
        return dir.listFiles().orEmpty().map { it.name.removeSuffix(".xml") }.filterNot(::foreign).sorted()
            .associateWith { context.getSharedPreferences(it, Context.MODE_PRIVATE).all.mapValues { (_, value) -> (value as? Set<*>)?.toSortedSet(compareBy { it.toString() }) ?: value } }
            .filterValues { it.isNotEmpty() }
    }

    fun forgetPreferences() = reset(emptyMap())

    private fun reset(wanted: Map<String, Map<String, Any?>>) {
        val dir = File(context.applicationInfo.dataDir, "shared_prefs")
        (dir.listFiles().orEmpty().map { it.name.removeSuffix(".xml") }.filterNot(::foreign) + wanted.keys).toSet().forEach { name ->
            val edit = context.getSharedPreferences(name, Context.MODE_PRIVATE).edit().clear()
            wanted[name].orEmpty().forEach { (key, value) ->
                when (value) {
                    is String -> edit.putString(key, value)
                    is Boolean -> edit.putBoolean(key, value)
                    is Int -> edit.putInt(key, value)
                    is Long -> edit.putLong(key, value)
                    is Float -> edit.putFloat(key, value)
                    is Set<*> -> edit.putStringSet(key, value.map { it.toString() }.toSet())
                }
            }
            edit.commit()
        }
    }

    fun all(): List<Probe<*>> {
        val manager = context.getSystemService(NotificationManager::class.java)
        val ui = context.getSystemService(UiModeManager::class.java)
        val power = context.getSystemService(PowerManager::class.java)
        val animation = ANIMATIONS.map { name -> Probe("animation $name", { global(name) }, { shell("settings put global $name $it") }) }
        val settings = KEYS.map { key -> Probe("setting $key", { Native.setting(key, null) }, { Native.setting(key, it) }) }
        return animation + settings + listOf(
            Probe(BATTERY, { shell("dumpsys battery").contains("UPDATES STOPPED") }, { shell("dumpsys battery reset") }),
            Probe(SAVER, { power.isPowerSaveMode }, { shell("cmd power set-mode ${if (it) 1 else 0}") }),
            Probe("do not disturb", { filter(manager) }, { shell("cmd notification set_dnd ${dnd(it)}") }),
            Probe("time zone", { java.util.TimeZone.getDefault().id }, { shell("service call alarm 3 s16 $it") }),
            Probe(NIGHT, { ui.nightMode }, { shell("cmd uimode night ${night(it)}") }),
            Probe("locale", { context.resources.configuration.locales.toLanguageTags() }, {}),
            Probe(LISTENER, { Players.allowed(context) }, { shell("cmd notification ${if (it) "allow_listener" else "disallow_listener"} $LISTENER_NAME") }),
            Probe(NOTICES, ::noticesAllowed, { if (it) Device.notifications(true) }),
            Probe("notifications", { manager.activeNotifications.map { it.id }.sorted() }, { manager.cancelAll() }),
            Probe("conversions", { Converting.progress.keys.sorted() }, { Converting.progress.keys.forEach(Converting::stop) }),
            Probe("hearing channel", { Heard.listening }, {}),
            Probe("open activities", { live() }, { finish() }),
            Probe(PREFERENCES, { prefs() }, ::reset),
        )
    }

    private fun finish() = InstrumentationRegistry.getInstrumentation().runOnMainSync {
        Stage.values().filter { it != Stage.DESTROYED }.forEach { stage ->
            ActivityLifecycleMonitorRegistry.getInstance().getActivitiesInStage(stage).forEach { it.finish() }
        }
    }

    private fun dnd(filter: Int) = when (filter) {
        NotificationManager.INTERRUPTION_FILTER_PRIORITY -> "priority"
        NotificationManager.INTERRUPTION_FILTER_ALARMS -> "alarms"
        NotificationManager.INTERRUPTION_FILTER_NONE -> "none"
        else -> "all"
    }

    private fun night(mode: Int) = when (mode) {
        UiModeManager.MODE_NIGHT_YES -> "yes"
        UiModeManager.MODE_NIGHT_NO -> "no"
        else -> "auto"
    }
}
