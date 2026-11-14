package io.github.wallpaper_rs

import android.graphics.Bitmap
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asAndroidBitmap
import org.json.JSONObject

object Shade {
    private const val WIDE = 96
    private const val TALL = 54
    private var shown: String? = null

    suspend fun follow(picture: ImageBitmap?) {
        val uid = if (picture == null) null else (ask("current") as? JSONObject)?.optString("on")?.ifEmpty { null }
        if (uid == shown) return
        shown = uid
        if (picture == null || uid == null) return Look.reset()
        Look.replace(tonesOf(JSONObject(Native.tones(pixels(picture), WIDE))))
    }

    private fun pixels(picture: ImageBitmap): IntArray {
        val source = picture.asAndroidBitmap().let { if (it.config == Bitmap.Config.HARDWARE) it.copy(Bitmap.Config.ARGB_8888, false) else it }
        val small = Bitmap.createScaledBitmap(source, WIDE, TALL, true)
        return IntArray(WIDE * TALL).also { small.getPixels(it, 0, WIDE, 0, 0, WIDE, TALL) }
    }
}

fun tonesOf(named: JSONObject): Tones {
    val tone = { key: String -> Color(android.graphics.Color.parseColor(named.getString(key))) }
    return Tones(
        night = tone("night"),
        nightHigh = tone("night_high"),
        nightLine = tone("night_line"),
        mist = tone("mist"),
        mistLow = tone("mist_low"),
        moon = tone("moon"),
        moonDeep = tone("moon_deep"),
        veil = named.getDouble("veil").toFloat(),
        frost = tone("frost"),
        amber = tone("amber"),
        coral = tone("coral"),
        sprout = tone("sprout"),
    )
}
