package io.github.wallpaper_rs

import android.graphics.BitmapFactory
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.unit.IntSize
import org.json.JSONObject

suspend fun posterOf(screen: IntSize): ImageBitmap? {
    val current = ask("current") as? JSONObject ?: return null
    val uid = current.takeUnless { it.isNull("on") }?.getString("on") ?: return null
    val item = Item(ask("item", "uid" to uid) as JSONObject)
    val whole = if (item.video || item.scene) null else decoded(item.path, screen)
    return whole ?: (ask("thumb", "uid" to uid) as? String)?.let(BitmapFactory::decodeFile)?.asImageBitmap()
}

fun decoded(path: String, screen: IntSize): ImageBitmap? {
    val bounds = BitmapFactory.Options().apply { inJustDecodeBounds = true }.also { BitmapFactory.decodeFile(path, it) }
    val fits = { scale: Int -> bounds.outWidth / (scale * 2) < screen.width || bounds.outHeight / (scale * 2) < screen.height }
    val scale = generateSequence(1) { it * 2 }.first(fits)
    return BitmapFactory.decodeFile(path, BitmapFactory.Options().apply { inSampleSize = scale })?.asImageBitmap()
}
