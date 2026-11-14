package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performImeAction
import androidx.compose.ui.test.performScrollToNode
import androidx.compose.ui.test.performTextReplacement
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class SettingsTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private val kept = mutableMapOf<String, String?>()

    @After
    fun closed() {
        scenario?.close()
        kept.forEach { (key, value) -> value?.let { Native.setting(key, it) } }
    }

    private fun shown(before: () -> Unit = {}) {
        storage()
        listOf(Updates.AUTO, "playback.quality", "transition.kind", LESS_MOTION).forEach { kept[it] = Native.setting(it, null) }
        before()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { Theme { Settings(main) } } }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag("finder")).fetchSemanticsNodes().isNotEmpty() }
    }

    private fun text(id: Int) = context.getString(id)

    private fun search(query: String) {
        compose.onNodeWithTag("finder").performTextReplacement(query)
        compose.onNodeWithTag("finder").performImeAction()
        compose.waitForIdle()
    }

    private fun row(label: Int) = compose.onNode(hasText(text(label)) and hasClickAction() and !hasTestTag("finder"))

    private fun top(tag: String) = compose.onNodeWithTag(tag, useUnmergedTree = true).fetchSemanticsNode().boundsInRoot.top

    @Test
    fun theFirstRowIsTheAccountAndTheThreeBlocksFollowInOrder() {
        shown()
        compose.onNodeWithTag("account").assertIsDisplayed()
        val list = compose.onNodeWithTag("settings")
        Block.entries.forEach { list.performScrollToNode(hasTestTag("block_${it.name}")) }
        list.performScrollToNode(hasTestTag("account"))
        assertTrue(top("account") < top("block_Wallpapers"))
        list.performScrollToNode(hasTestTag("more_Wallpapers"))
        compose.onNodeWithText(text(R.string.vote_counts)).assertDoesNotExist()
        compose.onNodeWithTag("more_Wallpapers").performClick()
        list.performScrollToNode(hasText(text(R.string.vote_counts)))
    }

    @Test
    fun everyRowHasALabelTheSearchFinds() {
        shown()
        SETTING_LINES.filter { it.always }.forEach { line ->
            search(text(line.label))
            compose.onNodeWithTag("nothing_found").assertDoesNotExist()
            compose.onNodeWithTag("settings").performScrollToNode(hasText(text(line.label)) and !hasTestTag("finder"))
            compose.onNodeWithTag("block_${line.block.name}").assertExists()
        }
    }

    @Test
    fun theSearchKeepsTheBlocksAndAnEmptySearchSaysSo() {
        shown()
        search(folded(text(R.string.pause_below)))
        compose.onNodeWithTag("block_Screen").assertExists()
        compose.onNodeWithTag("block_Wallpapers").assertDoesNotExist()
        compose.onNodeWithTag("account").assertDoesNotExist()
        search("zzqx")
        compose.onNodeWithTag("nothing_found").assertIsDisplayed()
        search("")
        compose.onNodeWithTag("account").assertIsDisplayed()
    }

    @Test
    fun qualityOffersThreeWordsAndTheSevenLevelsBehindMoreOptions() {
        shown()
        search(text(R.string.quality))
        row(R.string.quality).performClick()
        compose.onNodeWithText(text(R.string.quality_saving)).assertIsDisplayed()
        compose.onNodeWithText(text(R.string.level_q2)).assertDoesNotExist()
        compose.onNodeWithText(text(R.string.more_options)).performClick()
        compose.onNodeWithText(text(R.string.level_q2)).assertExists()
        compose.onNodeWithText(text(R.string.quality_top)).performClick()
        compose.waitUntil(TIMEOUT_MS) { Native.setting("playback.quality", null) == "q3" }
    }

    @Test
    fun turningOffTheAutomaticUpdatesSaysWhatItMeans() {
        shown { Native.setting(Updates.AUTO, "true") }
        search(text(R.string.auto_update))
        row(R.string.auto_update).performClick()
        compose.waitUntil(TIMEOUT_MS) { Native.setting(Updates.AUTO, null) == "false" }
        compose.onNodeWithText(text(R.string.auto_update_off)).assertIsDisplayed()
        assertEquals("false", Native.setting(Updates.AUTO, null))
    }

    @Test
    fun theTransitionRowSaysWhatTheChoiceDoesOnThisPhone() {
        shown {
            Native.setting("transition.kind", Passage.OWN)
            Native.setting(LESS_MOTION, "never")
            Native.setting("playback.quality", "q1")
        }
        search(text(R.string.fade))
        compose.onNodeWithText(text(R.string.passage_light)).assertIsDisplayed()
        row(R.string.fade).performClick()
        compose.waitUntil(TIMEOUT_MS) { Native.setting("transition.kind", null) == Passage.NONE }
        compose.onNodeWithText(text(R.string.passage_cut)).assertIsDisplayed()
    }

    private companion object {
        const val TIMEOUT_MS = 5_000L
    }
}
