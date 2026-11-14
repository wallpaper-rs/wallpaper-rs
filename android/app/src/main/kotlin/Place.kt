package io.github.wallpaper_rs

import android.app.WallpaperManager
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.Build
import android.widget.Toast
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.foundation.layout.padding
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource

const val BOTH = "both"
const val HOME = "home"
const val LOCK = "lock"

val TARGETS = listOf(Triple(HOME, R.string.home_screen, Ph.House), Triple(LOCK, R.string.lock_screen, Ph.LockSimple), Triple(BOTH, R.string.both_screens, Ph.DeviceMobile))

fun separate() = Build.VERSION.SDK_INT >= 34

fun Main.setWallpaper() {
    if (separate()) placing.value = true else place(BOTH)
}

fun assigned(target: String, flags: Int) = when (flags) {
    WallpaperManager.FLAG_SYSTEM -> HOME
    WallpaperManager.FLAG_LOCK -> LOCK
    else -> target
}

fun Context.announce(target: String) = Toast.makeText(this, done(target), Toast.LENGTH_LONG).show()

fun done(target: String) = when (target) {
    HOME -> R.string.done_home
    LOCK -> R.string.done_lock
    else -> R.string.done_both
}

fun Main.place(target: String) {
    placing.value = false
    if (active(target) && !Wallpaper.joined(target)) commit(target) else useAsWallpaper(target)
}

fun Main.placed(applied: Boolean) {
    val target = Wallpaper.commitWhenLive ?: return
    if (active(target)) commit(target) else if (!applied) Wallpaper.commitWhenLive = null
}

private fun Main.active(target: String): Boolean {
    val manager = WallpaperManager.getInstance(this)
    val home = manager.wallpaperInfo?.packageName == packageName
    if (!separate()) return Wallpaper.live > 0 || home
    val lock = (manager.getWallpaperInfo(WallpaperManager.FLAG_LOCK) ?: manager.wallpaperInfo)?.packageName == packageName
    return when (target) {
        HOME -> home
        LOCK -> lock
        else -> home && lock
    }
}

private fun Main.useAsWallpaper(target: String) {
    val wallpaper = ComponentName(this, Wallpaper::class.java)
    val intent = Intent(WallpaperManager.ACTION_CHANGE_LIVE_WALLPAPER)
        .putExtra(WallpaperManager.EXTRA_LIVE_WALLPAPER_COMPONENT, wallpaper)
    Wallpaper.commitWhenLive = target
    runCatching { preview.launch(intent) }.onFailure {
        Wallpaper.commitWhenLive = null
        toast(getString(R.string.unsupported))
    }
}

private fun Main.commit(target: String) = Render.post {
    Wallpaper.commitWhenLive = null
    Wallpaper.reload(Native.commit(target))
    runOnUiThread {
        revision.intValue++
        announce(target)
    }
}

@Composable
fun Placing(main: Main) = Sheet({ main.placing.value = false }, stringResource(R.string.first_where)) {
    TARGETS.forEach { (target, label, ph) ->
        SheetOption(stringResource(label), Modifier.testTag("place_$target"), ph) { main.place(target) }
    }
    WordAction({ main.placing.value = false }, Modifier.padding(horizontal = Measure.e2)) { Text(stringResource(R.string.cancel)) }
}
