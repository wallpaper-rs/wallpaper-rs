package io.github.wallpaper_rs

import android.app.Activity
import android.app.Instrumentation.ActivityResult
import android.app.WallpaperManager
import android.content.IntentFilter
import android.net.Uri
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@CleanApp
@RunWith(Baselined::class)
class FirstRunTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val instrumentation = InstrumentationRegistry.getInstrumentation()
    private val context = instrumentation.targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
        Wallpaper.commitWhenLive = null
        emptied()
    }

    private fun emptied() = runBlocking {
        val uids = fetch("items", "query" to "").map { it.uid }
        if (uids.isNotEmpty()) ask("remove", "uids" to uids)
    }

    private fun opened() {
        storage()
        emptied()
        assertTrue("nothing is in use", runBlocking { nowOn() } == null)
        scenario = ActivityScenario.launch(Main::class.java)
    }

    private fun shows(tag: String) = compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag(tag)).fetchSemanticsNodes().isNotEmpty() }

    private fun bundledName() = runBlocking { account("repos") as JSONObject }.getJSONArray("repos").let { all ->
        (0 until all.length()).map(all::getJSONObject).first { it.optBoolean("bundled") }.getString("name")
    }

    private fun placed(start: () -> Unit) {
        val monitor = instrumentation.addMonitor(IntentFilter(WallpaperManager.ACTION_CHANGE_LIVE_WALLPAPER), ActivityResult(Activity.RESULT_OK, null), true)
        try {
            start()
            imported()
            shows("first_$BOTH")
            compose.onNodeWithTag("first_$BOTH").performClick()
            compose.waitUntil(TIMEOUT_MS) { monitor.hits == 1 }
            assertEquals(BOTH, Wallpaper.commitWhenLive)
        } finally {
            instrumentation.removeMonitor(monitor)
        }
        Render.call { Native.commit(BOTH) }
        Wallpaper.commitWhenLive = null
        compose.waitUntil(TIMEOUT_MS) { runBlocking { nowOn() } != null }
    }

    private fun imported() = compose.waitUntil(SLOW_MS) {
        var idle = false
        scenario?.onActivity { idle = it.importing.intValue == 0 }
        idle && runBlocking { fetch("items", "query" to "") }.isNotEmpty()
    }

    @Test
    fun aCleanStartShowsTheExamplesAndTheirRepositoryTheFirstTime() {
        opened()
        compose.onNodeWithText(context.getString(R.string.empty_title)).assertIsDisplayed()
        shows("bundled")
        compose.onNodeWithTag("explore").performClick()
        shows("sources")
        val name = bundledName()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasText(name)).fetchSemanticsNodes().isNotEmpty() }
        compose.onNodeWithTag("connect_server").assertIsDisplayed()
    }

    @Test
    fun theExampleScenesBecomeTheFirstWallpaperAndTheNoticeAsksWhere() {
        opened()
        shows("add_examples")
        placed { compose.onNodeWithTag("add_examples").performClick() }
        compose.onNodeWithText(context.getString(R.string.first_where)).assertDoesNotExist()
    }

    @Test
    fun aPhotoOfThePersonBecomesTheFirstWallpaperWithTheSameQuestion() {
        opened()
        shows("explore")
        val photo = Uri.fromFile(still(0xff3a5a7a.toInt()))
        placed { scenario?.onActivity { it.store(listOf(photo)) } }
    }

    @Test
    fun theServerIsOneStepAwayFromTheRepositories() {
        opened()
        shows("explore")
        compose.onNodeWithTag("explore").performClick()
        shows("connect_server")
        compose.onNodeWithTag("connect_server").performClick()
        shows("address")
        shows("scan")
        scenario?.onActivity { it.onBackPressedDispatcher.onBackPressed() }
        shows("sources")
    }

    @Test
    fun theFirstWallpaperIsOfferedOnlyOnce() {
        opened()
        shows("add_examples")
        placed { compose.onNodeWithTag("add_examples").performClick() }
        scenario?.onActivity { it.examples() }
        Thread.sleep(QUIET_MS)
        assertTrue(compose.onAllNodes(hasTestTag("first_$BOTH")).fetchSemanticsNodes().isEmpty())
    }

    companion object {
        const val TIMEOUT_MS = 20_000L
        const val QUIET_MS = 3_000L
        const val SLOW_MS = 120_000L
    }

    @Test
    fun aFolderWithoutPicturesSaysWhichFolderWorksAndOffersAnother() {
        opened()
        shows("explore")
        instrumentation.runOnMainSync { emptyFolder.value = true }
        compose.onNodeWithText(context.getString(R.string.folder_empty_hint)).assertIsDisplayed()
        shows("choose_other")
        compose.onNodeWithText(context.getString(R.string.cancel)).performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag("choose_other")).fetchSemanticsNodes().isEmpty() }
        assertTrue(!emptyFolder.value)
    }
}
