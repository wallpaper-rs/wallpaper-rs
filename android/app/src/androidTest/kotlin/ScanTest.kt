package io.github.wallpaper_rs

import android.content.Context
import android.graphics.BitmapFactory
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.unit.dp
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

private const val PRINT = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

private class Fed(private val picture: String?, private val allow: Boolean) : Lens {
    override fun present(context: Context) = true

    override fun allowed(context: Context) = allow

    @Composable
    override fun asker(answer: (Boolean) -> Unit): () -> Unit = { answer(false) }

    @Composable
    override fun View(modifier: Modifier, seen: Sight) {
        Box(modifier.height(120.dp))
        LaunchedEffect(Unit) {
            val (frame, size) = luma(picture ?: return@LaunchedEffect)
            while (true) {
                withContext(Dispatchers.Default) { seen(frame, size) }
                delay(100)
            }
        }
    }
}

private fun luma(name: String): Pair<ByteArray, IntArray> {
    val picture = InstrumentationRegistry.getInstrumentation().context.assets.open(name).use(BitmapFactory::decodeStream)
    val (w, h) = picture.width to picture.height
    val pixels = IntArray(w * h).also { picture.getPixels(it, 0, w, 0, 0, w, h) }
    return ByteArray(w * h) { (pixels[it] shr 8 and 0xff).toByte() } to intArrayOf(w, h, w)
}

@RunWith(Baselined::class)
class ScanTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun shown(body: @Composable (Main) -> Unit) {
        storage()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Column { body(main) } }
        }
    }

    private fun waitFor(tag: String) =
        compose.waitUntil(10_000) { compose.onAllNodesWithTag(tag).fetchSemanticsNodes().isNotEmpty() }

    @Test
    fun aRefusedCameraLeavesTypingWithItsReason() {
        shown { main ->
            val desk = remember { Desk(main) }
            var scanning by remember { mutableStateOf(true) }
            if (scanning) {
                Scan(Fed(null, false), {}, { desk.blind = it; scanning = false }) {}
            } else {
                Connect(desk, {}, {}) {}
            }
        }
        waitFor("typing-stays")
        compose.onNodeWithText(context.getString(R.string.camera_refused)).assertIsDisplayed()
        compose.onNodeWithTag("address").assertIsDisplayed()
        compose.onNodeWithTag("scan").assertDoesNotExist()
    }

    @Test
    fun aForeignCodeSaysSoAndTheReaderKeepsLooking() {
        var found = false
        shown { Scan(Fed("code-foreign.png", true), { found = true }, {}) {} }
        waitFor("scan-line")
        compose.onNodeWithText(context.getString(R.string.error_android_connect_not_ours)).assertIsDisplayed()
        compose.onNodeWithTag("lens").assertIsDisplayed()
        assertFalse(found)
    }

    @Test
    fun aCodeOfOursConnectsWithItsServerAndFingerprint() {
        var called: Pair<String, String?>? = null
        shown { main ->
            val desk = remember { Desk(main).apply { dial = { called = url to print } } }
            Scan(Fed("code-ours.png", true), { desk.take(it, "phone") }, {}) {}
        }
        compose.waitUntil(10_000) { called != null }
        assertEquals("https://casa.example:8443" to PRINT, called)
    }
}
