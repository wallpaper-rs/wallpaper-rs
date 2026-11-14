package io.github.wallpaper_rs

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import androidx.compose.ui.unit.IntSize
import java.io.File
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class PosterTest {
    private var uid = ""

    @After
    fun closed() {
        if (uid.isNotEmpty()) runBlocking { ask("remove", "uids" to listOf(uid)) }
    }

    @Test
    fun theFrameBehindShowsTheWholePictureInUseNotItsThumbnail() = runBlocking {
        val file = File(storage(), "poster.jpg").apply {
            outputStream().use { Bitmap.createBitmap(WIDTH, HEIGHT, Bitmap.Config.ARGB_8888).apply { eraseColor(0xff6a4a2a.toInt()) }.compress(Bitmap.CompressFormat.JPEG, 90, it) }
        }
        install(file)
        uid = (ask("current") as JSONObject).getString("on")
        val thumb = BitmapFactory.decodeFile(ask("thumb", "uid" to uid) as String)
        val poster = posterOf(IntSize(SCREEN_W, SCREEN_H))!!
        assertEquals(WIDTH, poster.width)
        assertTrue("the poster is sharper than the thumbnail", poster.width > thumb.width)
        assertEquals(WIDTH / 2, posterOf(IntSize(SCREEN_W / 2, SCREEN_H / 2))!!.width)
    }

    private companion object {
        const val WIDTH = 3840
        const val HEIGHT = 2559
        const val SCREEN_W = 1080
        const val SCREEN_H = 2400
    }
}
