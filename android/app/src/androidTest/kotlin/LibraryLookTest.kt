package io.github.wallpaper_rs

import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.runtime.toMutableStateList
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.unit.dp
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class LibraryLookTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private val made = mutableListOf<String>()
    private val lists = mutableListOf<String>()
    private val before = runBlocking { ask("current") as? JSONObject }

    @After
    fun closed() {
        scenario?.close()
        runBlocking {
            lists.forEach { ask("delete", "playlist" to it) }
            if (made.isNotEmpty()) ask("remove", "uids" to made.toList())
            before?.let { was ->
                val back = when {
                    !was.isNull("playlist") -> choice("playlist", was.getString("playlist"))
                    !was.isNull("image") -> choice("item", was.getString("image"))
                    else -> null
                }
                back?.let { runCatching { ask("pick", "choice" to it) } }
            }
        }
    }

    private fun shown(body: @Composable (Main) -> Unit) {
        storage()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Column { body(main) } }
        }
    }

    private fun stored(index: Int): String = ParcelFileDescriptor.open(still(0xff304050.toInt() + index * 0x020202), ParcelFileDescriptor.MODE_READ_ONLY)
        .use { JSONObject(Native.store(it.fd, "Look $index.png")!!).getString("uid") }.also(made::add)

    private fun item(uid: String) = runBlocking { Item(ask("item", "uid" to uid) as JSONObject) }

    private fun text(id: Int, vararg args: Any) = compose.onNodeWithText(context.getString(id, *args))

    @Test
    fun theWallpaperInUseLeadsTheLibraryAndItsCoverSaysSo() {
        val item = item(stored(0))
        shown {
            Now(Showing(item, false), BOTH, {}, {})
            Cover(item, inUse = true)
        }
        text(R.string.lib_on_both).assertIsDisplayed()
        compose.onNodeWithTag("now").assertIsDisplayed()
        text(R.string.lib_options).assertIsDisplayed()
        compose.onNodeWithTag("in_use", useUnmergedTree = true).assertIsDisplayed()
        compose.onNodeWithTag("cover").assertIsDisplayed()
    }

    @Test
    fun theShelfFocusesOneCardThatOffersToBePutAndTheKindsLeadToTheGrid() {
        val items = List(3, ::stored).map(::item)
        var all = 0
        shown { main ->
            Kinds(0, {}) { all++ }
            Cards(main, items, null, 360.dp) {}
        }
        compose.onNodeWithTag("focused").assertIsDisplayed()
        text(R.string.apply).assertIsDisplayed()
        text(R.string.videos).assertIsDisplayed()
        text(R.string.lib_see_all).performClick()
        compose.waitForIdle()
        assertEquals(1, all)
    }

    @Test
    fun theCardInUseSaysItStays() {
        val item = item(stored(0))
        shown { main -> Cards(main, listOf(item), item.uid, 360.dp) {} }
        text(R.string.lib_stays).assertIsDisplayed()
        text(R.string.apply).assertDoesNotExist()
    }

    @Test
    fun aSearchWithNoMatchSaysWhatWasSearchedAndClearsIt() {
        var reset = false
        shown { Empty(it, Hollow.SEARCH, "bruma", { reset = true }) {} }
        text(R.string.lib_nothing_with, "bruma").assertIsDisplayed()
        text(R.string.clear_search).performClick()
        compose.waitForIdle()
        assertTrue(reset)
    }

    @Test
    fun noFavoritesOffersToSeeEverything() {
        var reset = false
        shown { Empty(it, Hollow.FAVORITES, reset = { reset = true }) {} }
        text(R.string.lib_no_favorites).assertIsDisplayed()
        text(R.string.lib_see_all).performClick()
        compose.waitForIdle()
        assertTrue(reset)
    }

    @Test
    fun theHomeLineShowsOnlyWhenLinkedAndOnline() {
        val linked = JSONObject().put("state", "connected").put("name", "casa")
        assertEquals("casa", linkedName(linked, true))
        assertNull(linkedName(linked, false))
        assertNull(linkedName(JSONObject().put("state", "error").put("name", "casa"), true))
        assertEquals("example.org", linkedName(JSONObject().put("state", "ready").put("url", "https://example.org/x"), true))
        var name by mutableStateOf<String?>(null)
        shown { Top(name, {}, {}) }
        compose.onNodeWithTag("connected").assertDoesNotExist()
        compose.onNodeWithContentDescription(context.getString(R.string.search)).assertIsDisplayed()
        compose.onNodeWithContentDescription(context.getString(R.string.choose)).assertIsDisplayed()
        name = "casa"
        text(R.string.lib_connected_to, "casa").assertIsDisplayed()
    }

    @Test
    fun theSelectionBarsCountAndHideTheRestUnderMore() {
        val uids = List(3, ::stored)
        shown { main -> Selecting(main, remember { uids.toMutableStateList() }, uids) }
        compose.onNodeWithText(context.resources.getQuantityString(R.plurals.lib_chosen, 3, 3)).assertIsDisplayed()
        compose.onNodeWithContentDescription(context.getString(R.string.cancel)).assertIsDisplayed()
        compose.onNodeWithContentDescription(context.getString(R.string.lib_choose_all)).assertIsDisplayed()
        listOf(R.string.lib_to_list, R.string.favorite, R.string.lib_rotate_these).forEach { text(it).assertIsDisplayed() }
        text(R.string.lib_more).performClick()
        text(R.string.fit).assertIsDisplayed()
        text(R.string.remove).assertIsDisplayed()
        text(R.string.unfavorite).assertIsDisplayed()
    }

    @Test
    fun rotatingTheChosenMakesAnHourlyListAndTheNoticeChangesIt() {
        val uids = List(3, ::stored)
        shown { main ->
            var done by remember { mutableStateOf<Made?>(null) }
            ChosenBar(main, remember { uids.toMutableStateList() }, null) { done = it }
            done?.let { Rotated(main, it) { done = null } }
        }
        val known = runBlocking { playlists() }.map { it.uid }.toSet()
        text(R.string.lib_rotate_these).performClick()
        val fresh = { runBlocking { playlists() }.filter { it.uid !in known } }
        assertTrue(waitFor { fresh().isNotEmpty() })
        val uid = fresh().single().uid.also(lists::add)
        assertTrue(fresh().single().name.startsWith(context.getString(R.string.rotation_name)))
        assertEquals(HOURLY, rules(uid).getString("interval"))
        assertEquals(3, runBlocking { playlists() }.single { it.uid == uid }.count)
        compose.waitUntil(10_000) { compose.onAllNodesWithTagCount("rotated") > 0 }
        text(R.string.lib_change_interval).performClick()
        text(INTERVALS.first { it.first == "6h" }.second).performClick()
        assertTrue(waitFor { rules(uid).optString("interval") == "6h" })
    }

    private fun rules(uid: String) = runBlocking { playlists() }.single { it.uid == uid }.rules

    private fun androidx.compose.ui.test.junit4.ComposeTestRule.onAllNodesWithTagCount(tag: String) =
        onAllNodes(androidx.compose.ui.test.hasTestTag(tag)).fetchSemanticsNodes().size
}
