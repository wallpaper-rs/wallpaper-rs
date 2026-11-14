package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.requiredSize
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasSetTextAction
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performScrollTo
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.unit.dp
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class FingerprintTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun keyboard(): Int {
        var height = 0
        scenario?.onActivity { main ->
            val insets = ViewCompat.getRootWindowInsets(main.window.decorView)
            height = if (insets?.isVisible(WindowInsetsCompat.Type.ime()) == true) insets.getInsets(WindowInsetsCompat.Type.ime()).bottom else 0
        }
        return height
    }

    @Test
    fun onTheSmallestScreenWithTheKeyboardOpenCheckStaysAboveItAndAnswers() {
        storage()
        val pin = "ab".repeat(32)
        var window = 0
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            val desk = Desk(main).apply { status = { JSONObject(mapOf("state" to "ready", "url" to "https://casa.example", "pin" to pin)) } }
            window = main.window.decorView.height
            main.setContent {
                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.BottomStart) {
                    Box(Modifier.requiredSize(SMALLEST_WIDTH.dp, SMALLEST_HEIGHT.dp)) { Print(desk) {} }
                }
            }
        }
        val field = compose.onNode(hasSetTextAction() and hasAnyAncestor(hasTestTag("print")))
        field.performClick()
        compose.waitUntil(TIMEOUT_MS) { keyboard() > 0 }
        field.performTextInput(pin)
        compose.waitForIdle()
        val check = compose.onNodeWithTag("check-print").assertIsDisplayed()
        val bottom = check.fetchSemanticsNode().boundsInWindow.bottom
        assertTrue("the button ends at $bottom, the keyboard starts at ${window - keyboard()}", bottom <= window - keyboard())
        check.performClick()
        compose.onNodeWithText(context.getString(R.string.print_same)).performScrollTo().assertIsDisplayed()
    }

    private companion object {
        const val SMALLEST_WIDTH = 320
        const val SMALLEST_HEIGHT = 480
        const val TIMEOUT_MS = 10_000L
    }
}
