package io.github.wallpaper_rs

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class FlashRouteTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
        Jump.find.value = null
    }

    @Test
    fun theFlashesMarkOpensSettingsAtTheRowThatAllowsThem() {
        storage()
        scenario = ActivityScenario.launch(Main::class.java)
        val row = context.getString(R.string.allow_flashing)
        scenario?.onActivity { allowIn(row) }
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag("finder")).fetchSemanticsNodes().isNotEmpty() }
        compose.onNode(hasText(row) and hasClickAction() and !hasTestTag("finder")).assertIsDisplayed()
    }

    private companion object {
        const val TIMEOUT_MS = 10_000L
    }
}
