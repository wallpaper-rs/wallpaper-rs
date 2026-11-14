package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onLast
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollToNode
import androidx.compose.ui.test.performTextInput
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import java.util.concurrent.atomic.AtomicInteger
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class ListsTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private val made = mutableListOf<String>()

    @After
    fun closed() {
        scenario?.close()
        Jump.to.value = null
        runBlocking { playlists().filter { it.uid in made || it.name in NAMES }.forEach { ask("delete", "playlist" to it.uid) } }
    }

    private fun words(id: Int, vararg args: Any) = context.getString(id, *args)

    private fun shown(text: String) = compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithText(text).fetchSemanticsNodes().isNotEmpty() }

    private fun fake(uid: String, name: String, every: String, count: Int, current: Boolean) = Playlist(
        JSONObject().put("count", count).put("current", current).put("away", org.json.JSONArray())
            .put("playlist", JSONObject().put("uid", uid).put("name", name).put("rules", JSONObject().put("interval", every))),
    )

    private fun show(content: @androidx.compose.runtime.Composable (Main) -> Unit) {
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { content(main) } }
    }

    @Test
    fun noListsSaysHowToMakeOneAndLeadsToTheLibrary() {
        show { Playlists(it, { emptyList() }) {} }
        shown(words(R.string.no_playlists))
        compose.onNodeWithText(words(R.string.lists_how)).assertIsDisplayed()
        compose.onNodeWithText(words(R.string.lists_go_library)).assertIsDisplayed().performClick()
        assertTrue(waitFor { Jump.to.value == Tab.Library })
    }

    @Test
    fun listsThatCannotBeReadSaySoAndTryAgain() {
        val tries = AtomicInteger()
        show { main ->
            Playlists(main, {
                if (tries.incrementAndGet() == 1) throw IllegalStateException("the library is locked")
                listOf(fake("again", "Again", "1h", 2, false))
            }) {}
        }
        shown(words(R.string.lists_unread))
        compose.onNodeWithText(words(R.string.lists_retry)).performClick()
        shown("Again")
        assertTrue(tries.get() >= 2)
    }

    @Test
    fun eachRowSaysInUseHowOftenAndHowManyAndANewListIsNamedInASheet() {
        val rows = listOf(fake("rotation", "Rotation", "1h", 3, true), fake("evening", "Evening", "30m", 5, false))
        show { Playlists(it, { rows }) {} }
        shown("Rotation")
        compose.onAllNodesWithText(words(R.string.in_use)).assertCountEquals(1)
        val plural = context.resources.getQuantityString(R.plurals.wallpapers, 3, 3)
        compose.onNodeWithText(words(R.string.every_1h) + " · " + plural).assertIsDisplayed()
        compose.onNodeWithText(words(R.string.new_playlist)).performClick()
        compose.onNodeWithText(words(R.string.name)).performTextInput(NAMES[0])
        compose.onNodeWithText(words(R.string.create)).performClick()
        assertTrue(waitFor { runBlocking { playlists().any { it.name == NAMES[0] } } })
    }

    @Test
    fun aListPageFoldsItsRareRulesChangesItsIntervalAndAsksBeforeDeleting() {
        val uid = runBlocking { ask("create", "name" to NAMES[1]) as String }.also { made += it }
        var closed = false
        show { Entries(it, uid, {}) { closed = true } }
        shown(words(R.string.list_empty))
        compose.onNodeWithText(words(R.string.list_empty_how)).assertIsDisplayed()
        val list = compose.onNodeWithTag("list-page")
        list.performScrollToNode(hasText(words(R.string.list_more)))
        compose.onAllNodesWithText(words(R.string.resume_list)).assertCountEquals(0)
        compose.onNodeWithText(words(R.string.list_more)).performClick()
        list.performScrollToNode(hasText(words(R.string.resume_list)))
        list.performScrollToNode(hasText(words(R.string.change)))
        compose.onNodeWithText(words(R.string.change)).performClick()
        compose.onAllNodesWithText(words(R.string.every_1h)).onLast().performClick()
        assertTrue(waitFor { runBlocking { playlists().first { it.uid == uid }.rules.optString("interval") == "1h" } })
        list.performScrollToNode(hasText(words(R.string.delete)))
        compose.onNodeWithText(words(R.string.delete)).performClick()
        shown(words(R.string.list_delete_ask, NAMES[1]))
        compose.onAllNodesWithText(words(R.string.delete)).onLast().performClick()
        assertTrue(waitFor { closed && runBlocking { playlists().none { it.uid == uid } } })
    }

    private companion object {
        const val TIMEOUT_MS = 5_000L
        val NAMES = listOf("Made in a sheet", "Folded rules")
    }
}
