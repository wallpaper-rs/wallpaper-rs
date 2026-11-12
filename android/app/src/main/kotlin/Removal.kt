package io.github.wallpaper_rs

import android.app.WallpaperManager
import android.content.ComponentName
import android.content.Intent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONObject

private fun Main.handBack() {
    val manager = WallpaperManager.getInstance(this)
    runCatching { manager.clear(WallpaperManager.FLAG_SYSTEM or WallpaperManager.FLAG_LOCK) }
}

private fun Main.chooseAgain() {
    val wallpaper = ComponentName(this, Wallpaper::class.java)
    val intent = Intent(WallpaperManager.ACTION_CHANGE_LIVE_WALLPAPER)
        .putExtra(WallpaperManager.EXTRA_LIVE_WALLPAPER_COMPONENT, wallpaper)
    Wallpaper.commitWhenLive = null
    runCatching { preview.launch(intent) }.onFailure { toast(getString(R.string.unsupported)) }
}

@Composable
fun Removal(main: Main) = Column {
    var changes by remember { mutableIntStateOf(0) }
    val state by loaded(JSONObject(), changes) { ask("wallpaper") as JSONObject }
    Header(stringResource(R.string.wallpaper))
    TextButton(
        {
            main.act {
                ask("remove_wallpaper")
                main.handBack()
                changes++
                main.toast(main.getString(R.string.wallpaper_removed))
            }
        },
        Modifier.padding(horizontal = 8.dp),
        enabled = !state.optBoolean("blank"),
    ) { Text(stringResource(R.string.remove_wallpaper)) }
    TextButton(
        {
            main.act {
                ask("restore_wallpaper")
                changes++
                main.chooseAgain()
            }
        },
        Modifier.padding(horizontal = 8.dp),
        enabled = state.optBoolean("previous"),
    ) { Text(stringResource(R.string.bring_back_wallpaper)) }
    Note(stringResource(R.string.remove_wallpaper_note))
}
