package io.github.wallpaper_rs

import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.requiredSize
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.test.SemanticsNodeInteraction
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.unit.dp
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class SmallPageTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null
    private var uid = ""

    @After
    fun closed() {
        scenario?.close()
        if (uid.isNotEmpty()) runBlocking { ask("remove", "uids" to listOf(uid)) }
    }

    private fun bounds(node: SemanticsNodeInteraction): Rect = node.fetchSemanticsNode().boundsInRoot

    private fun whole(inner: Rect, outer: Rect) =
        inner.left >= outer.left - 1 && inner.top >= outer.top - 1 && inner.right <= outer.right + 1 && inner.bottom <= outer.bottom + 1

    @Test
    fun theDepthMapButtonIsWholeAndPressableOnTheSmallestScreen() {
        val file = still(0xff557799.toInt())
        uid = ParcelFileDescriptor.open(file, ParcelFileDescriptor.MODE_READ_ONLY).use { JSONObject(Native.store(it.fd, "Small.png")!!).getString("uid") }
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Box(Modifier.requiredSize(320.dp, 480.dp).testTag("small")) { Detail(main, uid, {}, {}) } }
        }
        val make = hasText(context.getString(R.string.make_map)) and hasClickAction()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(make).fetchSemanticsNodes().isNotEmpty() }
        val button = compose.onNode(make).performScrollTo()
        compose.waitForIdle()
        val screen = bounds(compose.onNodeWithTag("small"))
        assertTrue("the depth map button is whole", whole(bounds(button), screen))
        val apply = compose.onNode(hasText(context.getString(R.string.apply)) and hasClickAction(), useUnmergedTree = false)
        assertTrue("the foot stays whole", whole(bounds(apply), screen))
        button.performClick()
        compose.waitForIdle()
        assertTrue(compose.onAllNodesWithText(context.getString(R.string.make_map)).fetchSemanticsNodes().isNotEmpty())
    }

    private companion object {
        const val TIMEOUT_MS = 10_000L
    }
}
