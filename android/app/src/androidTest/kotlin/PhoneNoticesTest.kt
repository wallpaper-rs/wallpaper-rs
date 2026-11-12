package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
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
    fun anEmptyLibraryOffersTheExampleScenesAndAPickerRightThere() {
        shown { Empty(it, Hollow.LIBRARY) {} }
        text(R.string.empty).assertIsDisplayed()
        text(R.string.examples).assertIsDisplayed()
        text(R.string.pick_media).assertIsDisplayed()
    }

    @Test
    fun theWebFilterTalksAboutWebPagesAndOffersNoExamples() {
        shown { Empty(it, Hollow.WEBS) {} }
        text(R.string.empty_webs).assertIsDisplayed()
        text(R.string.examples).assertDoesNotExist()
        text(R.string.empty).assertDoesNotExist()
    }

    @Test
    fun aServerAddressThatIsNotOneAnswersOnTheScreen() {
        shown { main -> Server(main) { _, work -> main.act(false) { work() } } }
        text(R.string.add_server).performClick()
        text(R.string.server_blank).assertIsDisplayed()
    }

    @Test
    fun aSwitchIsOneControlWithItsName() {
        shown { Toggle(context.getString(R.string.fade), true) {} }
        text(R.string.fade).assertIsOn()
        text(R.string.fade).performClick()
        text(R.string.fade).assertIsOn()
    }
}
