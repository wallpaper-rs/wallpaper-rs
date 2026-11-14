package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class AsksTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private var uid = ""

    @Before
    fun installed() {
        val dir = File(storage(), "asked/asked").apply { deleteRecursively(); mkdirs() }
        File(dir, "scene.json").writeText(MEDIA_SCENE)
        File(dir, "manifest.json").writeText(MEDIA_MANIFEST.replace("org.example.tinted", "org.example.asked${System.nanoTime()}"))
        uid = JSONArray(Native.library(JSONObject().put("op", "examples").put("dir", dir.parent).toString())).getString(0)
    }

    @After
    fun closed() {
        scenario?.close()
        runBlocking { ask("remove", "uids" to listOf(uid)) }
    }

    private fun granted() = runBlocking { (ask("scene", "uid" to uid, "languages" to listOf<String>()) as JSONObject).getJSONObject("granted").getBoolean("media") }

    private fun answer(word: Int): List<String> {
        assertEquals(listOf("media"), runBlocking { asked(uid) })
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent {
                val undecided by loaded(emptyList<String>(), main.revision.intValue) { asked(uid) }
                Column(Modifier.verticalScroll(rememberScrollState())) { Asks(main, uid, undecided) }
            }
        }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasText(context.getString(word))).fetchSemanticsNodes().isNotEmpty() }
        compose.onNodeWithTag("ask_media").performScrollTo()
        compose.onNode(hasText(context.getString(word))).performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasText(context.getString(R.string.asks_title))).fetchSemanticsNodes().isEmpty() }
        return runBlocking { asked(uid) }
    }

    @Test
    fun withoutItIsRememberedAndTheWallpaperPlaysWithoutIt() {
        assertEquals(emptyList<String>(), answer(R.string.asks_without))
        assertEquals(false, granted())
    }

    @Test
    fun allowIsRememberedAndGiven() {
        assertEquals(emptyList<String>(), answer(R.string.asks_allow))
        assertEquals(true, granted())
    }

    private companion object {
        const val TIMEOUT_MS = 10_000L
    }
}
