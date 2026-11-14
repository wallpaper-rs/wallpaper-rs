package io.github.wallpaper_rs

import android.content.res.Configuration
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsSelected
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import androidx.core.view.WindowCompat
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class ShellTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun opened() {
        storage()
        scenario = ActivityScenario.launch(Main::class.java)
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodes(hasTestTag("tabs")).fetchSemanticsNodes().isNotEmpty() }
    }

    private fun back() {
        scenario?.onActivity { it.onBackPressedDispatcher.onBackPressed() }
        compose.waitForIdle()
    }

    private fun page(label: Int) = compose.onNode(hasText(context.getString(label)) and !hasAnyAncestor(hasTestTag("tabs")))

    private fun tab(label: Int) = compose.onNode(hasText(context.getString(label)) and hasAnyAncestor(hasTestTag("tabs")) and isTab)

    @Test
    fun theBarHoldsThreeWordsAndEachOneOpensItsPage() {
        opened()
        compose.onAllNodes(isTab).assertCountEquals(3)
        tab(R.string.library).assertIsSelected()
        tab(R.string.playlists).performClick().assertIsSelected()
        page(R.string.playlists).assertIsDisplayed()
        tab(R.string.settings).performClick().assertIsSelected()
        compose.onNodeWithText(context.getString(R.string.account)).assertIsDisplayed()
    }

    @Test
    fun theAccountRowOpensTheAccountPageAndBackReturnsToTheSettings() {
        opened()
        tab(R.string.settings).performClick()
        compose.onNodeWithText(context.getString(R.string.account)).performClick()
        compose.onNodeWithText(context.getString(R.string.not_connected)).assertIsDisplayed()
        compose.onNodeWithTag("tabs").assertDoesNotExist()
        back()
        compose.onNodeWithTag("tabs").assertIsDisplayed()
        tab(R.string.settings).assertIsSelected()
    }

    @Test
    fun fromInternetInTheAddMenuOpensTheSourcesPage() {
        opened()
        compose.onNodeWithContentDescription(context.getString(R.string.choose)).performClick()
        compose.onNodeWithText(context.getString(R.string.from_internet)).performClick()
        compose.onNodeWithText(context.getString(R.string.sources)).assertIsDisplayed()
        back()
        tab(R.string.library).assertIsSelected()
    }

    @Test
    fun theSystemBarsKeepLightIconsWhileThePhoneIsInLightMode() {
        val night = context.resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK
        assertEquals(Configuration.UI_MODE_NIGHT_NO, night)
        opened()
        scenario?.onActivity { main ->
            val bars = WindowCompat.getInsetsController(main.window, main.window.decorView)
            assertFalse(bars.isAppearanceLightStatusBars)
            assertFalse(bars.isAppearanceLightNavigationBars)
        }
    }

    private companion object {
        const val TIMEOUT_MS = 10_000L
        val isTab = SemanticsMatcher.expectValue(SemanticsProperties.Role, Role.Tab)
    }
}
