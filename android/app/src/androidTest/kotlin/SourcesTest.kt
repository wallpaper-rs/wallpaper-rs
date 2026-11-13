package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsOff
import androidx.compose.ui.test.hasScrollAction
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollToNode
import androidx.compose.ui.test.performTextInput
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@CleanApp
@RunWith(Baselined::class)
class SourcesTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
        runBlocking {
            val left = (account("providers") as JSONObject).getJSONArray("providers")
            (0 until left.length()).forEach { account("provider_remove", "id" to left.getJSONObject(it).getString("id"), "purge" to true) }
        }
    }

    private fun shown(content: @androidx.compose.runtime.Composable (Main) -> Unit) {
        storage()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { content(main) } }
    }

    private fun text(id: Int, vararg args: Any) = compose.onNodeWithText(context.getString(id, *args))

    private fun reached(text: String) = compose.onNode(hasScrollAction()).performScrollToNode(hasText(text)).let { compose.onNode(hasText(text)) }

    private fun patient(done: () -> Boolean): Boolean {
        val end = System.currentTimeMillis() + PATIENCE_MS
        while (!done() && System.currentTimeMillis() < end) Thread.sleep(100)
        return done()
    }

    private fun providers() = runBlocking { account("providers") as JSONObject }.getJSONArray("providers")

    @Test
    fun anEmptyLibraryPointsToTheSourcesAndWithoutAnyServer() {
        var opened = 0
        shown { Empty(it, Hollow.LIBRARY) { opened++ } }
        text(R.string.get_wallpapers).assertIsDisplayed().performClick()
        assertEquals(1, opened)
    }

    @Test
    fun theSourcesPageOffersPicturesAndRepositoriesWithNothingSetUp() {
        shown { Sources(it) {} }
        text(R.string.sources).assertIsDisplayed()
        text(R.string.providers).assertIsDisplayed()
        reached(context.getString(kindTitle("bing"))).assertIsDisplayed()
        reached(context.getString(R.string.repos_none)).assertIsDisplayed()
        reached(context.getString(R.string.repo_fingerprint)).assertIsDisplayed()
        assertEquals(0, providers().length())
    }

    @Test
    fun aRepositoryNeverAddsWithoutItsFingerprintAndSaysWhy() {
        shown { main -> RepoAdd(main, { _, work -> main.act(false) { work() } }) {} }
        compose.onNode(hasText(context.getString(R.string.repo_url))).performTextInput("https://example.org/wallpapers/")
        text(R.string.repo_add_url).performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasText(context.getString(R.string.error_android_repo_needs_fingerprint))).fetchSemanticsNodes().isNotEmpty() }
        text(R.string.repo_sent, "example.org").assertIsDisplayed()
    }

    @Test
    fun aPictureSourceStaysOffAndSilentUntilTheUserTurnsItOn() {
        FakeDav().use { dav ->
            runBlocking {
                account("provider_add", "kind" to "webdav", "options" to JSONObject().put("url", dav.url).put("user", "me"), "secret" to "pw", "on" to false)
            }
            shown { Sources(it) {} }
            reached(context.getString(R.string.sent_dav, "127.0.0.1")).assertIsDisplayed()
            text(R.string.provider_never).assertIsDisplayed()
            reached(context.getString(R.string.provider_turn_on)).assertIsOff()
            Thread.sleep(SILENCE_MS)
            assertEquals("nothing is contacted while it is off", 0, dav.listings.get() + dav.downloads.get())
            compose.onNode(hasText(context.getString(R.string.provider_turn_on))).performClick()
            assertTrue(patient { dav.downloads.get() == 3 })
            val counted = patient { providers().getJSONObject(0).optInt("count") == 3 }
            assertTrue("what the source brought: ${providers()}", counted)
            assertTrue(providers().getJSONObject(0).getBoolean("on"))
            val brought = runBlocking { ask("items", "query" to "tag:from-webdav") as JSONArray }
            assertEquals(3, brought.length())
            runBlocking { account("provider_remove", "id" to "webdav", "purge" to true) }
            assertEquals(0, (runBlocking { ask("items", "query" to "tag:from-webdav") } as JSONArray).length())
        }
    }

    private companion object {
        const val TIMEOUT_MS = 10_000L
        const val SILENCE_MS = 1_000L
        const val PATIENCE_MS = 120_000L
    }
}
