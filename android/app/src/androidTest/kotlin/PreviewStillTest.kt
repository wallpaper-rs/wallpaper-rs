package io.github.wallpaper_rs

import android.graphics.Bitmap
import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.test.core.app.ActivityScenario
import java.io.File
import java.util.concurrent.CountDownLatch
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class PreviewStillTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private var scenario: ActivityScenario<Main>? = null
    private val release = CountDownLatch(1)
    private var uid = ""

    @After
    fun closed() {
        release.countDown()
        scenario?.close()
        if (uid.isNotEmpty()) runBlocking { ask("remove", "uids" to listOf(uid)) }
    }

    @Test
    fun theThumbnailOfALargePhotoShowsBeforeItsFullPictureIsDecoded() {
        val file = File(storage(), "large.jpg").apply {
            outputStream().use { out ->
                Bitmap.createBitmap(WIDTH, HEIGHT, Bitmap.Config.ARGB_8888).apply { eraseColor(0xff4a6a8a.toInt()) }
                    .compress(Bitmap.CompressFormat.JPEG, 90, out)
            }
        }
        uid = ParcelFileDescriptor.open(file, ParcelFileDescriptor.MODE_READ_ONLY).use { JSONObject(Native.store(it.fd, "Large.jpg")!!).getString("uid") }
        Render.post { release.await() }
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { Detail(main, uid, {}, {}) } }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithTag("still").fetchSemanticsNodes().isNotEmpty() }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithTag("unpainted").fetchSemanticsNodes().isNotEmpty() }
        release.countDown()
        compose.waitUntil(TIMEOUT_MS * 4) { compose.onAllNodesWithTag("unpainted").fetchSemanticsNodes().isEmpty() }
    }

    private companion object {
        const val WIDTH = 3840
        const val HEIGHT = 2559
        const val TIMEOUT_MS = 10_000L
    }
}
