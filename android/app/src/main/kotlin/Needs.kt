package io.github.wallpaper_rs

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

suspend fun Main.complete(reply: Any?) {
    val failed = (reply as? JSONObject)?.optJSONArray("failed")?.strings().orEmpty()
    if (failed.isNotEmpty()) withContext(Dispatchers.Main) { toast(getString(R.string.needs_failed, failed.joinToString(", "))) }
}
