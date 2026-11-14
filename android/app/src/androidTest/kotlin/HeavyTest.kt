package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
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
class HeavyTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private var uid = ""

    @Before
    fun installed() {
        val dir = File(storage(), "heavy/heavy").apply { deleteRecursively(); mkdirs() }
        File(dir, "scene.json").writeText(MEDIA_SCENE)
        File(dir, "manifest.json").writeText(MEDIA_MANIFEST.replace("org.example.tinted", "org.example.heavy${System.nanoTime()}"))
        uid = JSONArray(Native.library(JSONObject().put("op", "examples").put("dir", dir.parent).toString())).getString(0)
    }

    @After
    fun closed() {
        scenario?.close()
        runBlocking { ask("remove", "uids" to listOf(uid)) }
    }

    private fun shown(report: JSONObject) {
        scenario?.close()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent {
                Column {
                    HeavyNotice(main, uid, TITLE, report)
                    Standing(report)
                    if (heavy(report)) PlayAnyway(main)
                }
            }
        }
        compose.waitForIdle()
    }

    private fun gone(tag: String) = compose.onAllNodes(hasTestTag(tag)).fetchSemanticsNodes().isEmpty()

    @Test
    fun aStillSceneSaysWhyAndPlaysAnywayAtTheLowestLevelOnce() {
        Device.sanctioned += "setting playback.quality"
        shown(JSONObject("""{ "auto": "still" }"""))
        compose.waitUntil(TIMEOUT_MS) { !gone("heavy_notice") }
        compose.onNodeWithText(context.getString(R.string.heavy_notice, TITLE)).assertIsDisplayed()
        compose.onNodeWithText(context.getString(R.string.still_heavy)).assertIsDisplayed()
        compose.onNodeWithTag("play_anyway").assertIsDisplayed().performClick()
        compose.waitUntil(TIMEOUT_MS) { Native.setting("playback.quality", null) == "min-q1" }
        shown(JSONObject("""{ "auto": "still" }"""))
        compose.onNodeWithTag("play_anyway").assertIsDisplayed()
        assertEquals(true, gone("heavy_notice"))
    }

    @Test
    fun aSceneThatPlaysShowsNoNoticeAndNoButton() {
        shown(JSONObject("""{ "auto": "playing" }"""))
        assertEquals(listOf(true, true, true), listOf(gone("heavy_notice"), gone("play_anyway"), gone("standing")))
        assertEquals(true, runBlocking { ask("tell", "uid" to uid) })
    }

    private companion object {
        const val TITLE = "Solids"
        const val TIMEOUT_MS = 10_000L
    }
}
