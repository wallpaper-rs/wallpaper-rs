package io.github.wallpaper_rs

import android.app.WallpaperManager
import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import android.net.Uri
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import org.json.JSONObject

class Showing(val item: Item, val listed: Boolean)

suspend fun showing(languages: List<String>): Showing? {
    val current = ask("current") as? JSONObject ?: return null
    val uid = current.takeUnless { it.isNull("on") }?.optString("on")?.takeIf(String::isNotEmpty) ?: return null
    val item = runCatching { Item(ask("item", "uid" to uid, "languages" to languages) as JSONObject) }.getOrNull() ?: return null
    return Showing(item, !current.isNull("playlist"))
}

private val LINKED = setOf("connected", "needs_key", "paused", "ready")

fun linkedName(status: JSONObject, online: Boolean): String? = status.takeIf { online && it.optString("state") in LINKED }
    ?.let { it.optString("name").ifBlank { Uri.parse(it.optString("url")).host.orEmpty() } }?.takeIf(String::isNotBlank)

fun Context.online(): Boolean = getSystemService(ConnectivityManager::class.java)
    ?.let { it.getNetworkCapabilities(it.activeNetwork) }?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true

fun Context.screens(): String? {
    val manager = WallpaperManager.getInstance(this)
    val home = manager.wallpaperInfo?.packageName == packageName
    val lock = if (separate()) (manager.getWallpaperInfo(WallpaperManager.FLAG_LOCK) ?: manager.wallpaperInfo)?.packageName == packageName else home
    return when {
        home && lock -> BOTH
        home -> HOME
        lock -> LOCK
        else -> null
    }
}

private val ON = mapOf(HOME to R.string.lib_on_home, LOCK to R.string.lib_on_lock, BOTH to R.string.lib_on_both)
private val ROTATING = mapOf(HOME to R.string.lib_rotating_home, LOCK to R.string.lib_rotating_lock, BOTH to R.string.lib_rotating_both)

@Composable
fun Top(linked: String?, search: () -> Unit, add: () -> Unit, searching: Boolean = false) = Row(
    Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.e3, top = 6.dp).heightIn(min = Measure.touch),
    Arrangement.spacedBy(Measure.e2),
    Alignment.CenterVertically,
) {
    Row(Modifier.weight(1f), Arrangement.spacedBy(7.dp), Alignment.CenterVertically) {
        linked?.let { name ->
            Glyph(Ph.CloudCheck, null, tint = tones.frost, size = Measure.iconFold)
            Text(stringResource(R.string.lib_connected_to, name), Modifier.testTag("connected"), tones.frost, style = Type.foot)
        }
    }
    Square(search, Ph.MagnifyingGlass, stringResource(R.string.search), lit = searching)
    Square(add, Ph.Plus, stringResource(R.string.choose))
}

@Composable
fun Now(now: Showing, where: String?, options: () -> Unit, favorite: (Boolean) -> Unit) =
    Column(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e8, bottom = Measure.e6).testTag("now")) {
        where?.let { Text(stringResource((if (now.listed) ROTATING else ON).getValue(it)), color = tones.frost, style = Type.text) }
        Text(now.item.title, style = Type.titleL)
        Text(facts(now.item), Modifier.padding(top = Measure.e2), tones.frost, style = Type.text.copy(fontWeight = FontWeight.Medium))
        val shape = RoundedCornerShape(Measure.rControl)
        Row(
            Modifier.padding(top = Measure.e5).height(IntrinsicSize.Min).clip(shape).background(tones.veiled(MANDOS), shape).border(Measure.line, tones.nightLine, shape),
        ) {
            Mando(Ph.SlidersHorizontal, stringResource(R.string.lib_options), false, options)
            Box(Modifier.width(Measure.line).fillMaxHeight().background(tones.nightLine))
            val loved = now.item.favorite
            Mando(Ph.Heart, stringResource(R.string.favorite), loved) { favorite(!loved) }
        }
    }

@Composable
private fun Mando(ph: Ph, label: String, lit: Boolean, onClick: () -> Unit) = Row(
    Modifier.clickable(role = Role.Button, onClick = onClick).heightIn(min = Measure.touch).padding(horizontal = Measure.e4),
    Arrangement.spacedBy(9.dp),
    Alignment.CenterVertically,
) {
    Glyph(ph, null, tint = tones.frost, lit = lit, size = Measure.iconSmall)
    Text(label, color = tones.frost, style = Type.text.copy(fontWeight = FontWeight.Medium))
}

private const val MANDOS = 0.82f
