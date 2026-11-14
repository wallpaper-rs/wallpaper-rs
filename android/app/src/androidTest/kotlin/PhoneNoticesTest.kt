package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.hasSetTextAction
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class PhoneNoticesTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun shown(body: @androidx.compose.runtime.Composable (Main) -> Unit) {
        storage()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Column { body(main) } }
        }
    }

    private fun text(id: Int) = compose.onNodeWithText(context.getString(id))

    @Test
    fun anEmptyLibraryOffersTheRepositoriesFirstAndThePhotosSecond() {
        shown { Empty(it, Hollow.LIBRARY) {} }
        text(R.string.empty_title).assertIsDisplayed()
        text(R.string.empty).assertIsDisplayed()
        val explore = compose.onNodeWithText(context.getString(R.string.explore_repos)).assertIsDisplayed().fetchSemanticsNode().boundsInRoot
        val photos = compose.onNodeWithText(context.getString(R.string.add_photos)).assertIsDisplayed().fetchSemanticsNode().boundsInRoot
        assertTrue("explore comes first", explore.top < photos.top)
    }

    @Test
    fun theWebFilterTalksAboutWebPagesAndOffersNoExamples() {
        shown { Empty(it, Hollow.WEBS) {} }
        text(R.string.empty_webs).assertIsDisplayed()
        text(R.string.add_them).assertDoesNotExist()
        text(R.string.empty).assertDoesNotExist()
    }

    @Test
    fun aServerAddressThatIsNotOneAnswersOnTheScreen() {
        shown { main -> Connect(Desk(main), {}, {}) {} }
        compose.onAllNodes(hasSetTextAction())[0].performTextInput("not an address")
        compose.onNodeWithTag("connect").performClick()
        compose.waitUntil(10_000) { compose.onAllNodesWithTag("problem").fetchSemanticsNodes().isNotEmpty() }
    }

    @Test
    fun aSwitchIsOneControlWithItsName() {
        shown { Toggle(context.getString(R.string.fade), true) {} }
        text(R.string.fade).assertIsOn()
        text(R.string.fade).performClick()
        text(R.string.fade).assertIsOn()
    }
}
