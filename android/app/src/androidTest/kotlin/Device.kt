package io.github.wallpaper_rs

import android.content.Context
import android.os.ParcelFileDescriptor
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assume

object Device {
    private val instrumentation get() = InstrumentationRegistry.getInstrumentation()
    val context: Context get() = instrumentation.targetContext
    val sanctioned = mutableSetOf<String>()

    fun shell(command: String): String =
        ParcelFileDescriptor.AutoCloseInputStream(instrumentation.uiAutomation.executeShellCommand(command)).use { String(it.readBytes()) }

    fun unplugged(level: Int? = null) {
        sanctioned += Probes.BATTERY
        shell("dumpsys battery unplug")
        level?.let { shell("dumpsys battery set level $it") }
    }

    fun batteryReset() {
        sanctioned += Probes.BATTERY
        shell("dumpsys battery reset")
    }

    fun saving(on: Boolean) {
        sanctioned += Probes.SAVER
        shell("cmd power set-mode ${if (on) 1 else 0}")
    }

    fun animations(scale: String) {
        sanctioned += Probes.ANIMATIONS.map { "animation $it" }
        Probes.ANIMATIONS.forEach { shell("settings put global $it $scale") }
    }

    fun listener(on: Boolean) {
        sanctioned += Probes.LISTENER
        shell("cmd notification ${if (on) "allow_listener" else "disallow_listener"} ${Probes.LISTENER_NAME}")
    }

    fun nightMode(mode: String) {
        sanctioned += Probes.NIGHT
        shell("cmd uimode night $mode")
    }

    fun notifications(on: Boolean) {
        sanctioned += Probes.NOTICES
        val pkg = context.packageName
        if (on) {
            instrumentation.uiAutomation.grantRuntimePermission(pkg, android.Manifest.permission.POST_NOTIFICATIONS)
        } else {
            Assume.assumeFalse("revoking a granted permission kills the app process", Probes.noticesAllowed())
        }
    }

    fun normalize() {
        batteryReset()
        saving(false)
        listener(false)
        powerReset()
    }

    fun powerReset() = Native.power(false, false, false, 0, 0f, false, -1)
}
