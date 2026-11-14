package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.ui.test.hasParent
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onRoot
import androidx.compose.ui.test.performImeAction
import androidx.compose.ui.test.performTextReplacement
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class SettingRowLookTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private var kept: String? = null

    @After
    fun closed() {
        scenario?.close()
        kept?.let { Native.setting(PACE, it) }
    }

    @Test
    fun aLongChoiceLeavesItsNameReadableAndApart() {
        storage()
        kept = Native.setting(PACE, null)
        Native.setting(PACE, "detail")
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { Theme { Settings(main) } } }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag("finder")).fetchSemanticsNodes().isNotEmpty() }
        val name = context.getString(R.string.auto_pace)
        compose.onNode(hasTestTag("finder")).performTextReplacement(name)
        compose.onNode(hasTestTag("finder")).performImeAction()
        compose.waitForIdle()
        val label = compose.onNode(hasText(name) and hasParent(hasTestTag("setting_name")), useUnmergedTree = true).fetchSemanticsNode()
        val column = label.parent!!.boundsInRoot
        val value = compose.onNode(hasTestTag("setting_value") and hasText(context.getString(R.string.pace_detail)), useUnmergedTree = true)
            .fetchSemanticsNode().boundsInRoot
        val width = compose.onRoot().fetchSemanticsNode().size.width
        assertTrue("name column $column of $width", column.width > width * SHARE)
        assertTrue("value $value overlaps name ${label.boundsInRoot}", value.top >= label.boundsInRoot.bottom || value.left >= label.boundsInRoot.right)
    }

    private companion object {
        const val PACE = "playback.auto_pace"
        const val SHARE = 0.4f
        const val TIMEOUT_MS = 5_000L
    }
}
