package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RepoUpdatesTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
        Native.power(false, false, false, 0, 0f, false)
    }

    private fun shown(content: @androidx.compose.runtime.Composable (Main) -> Unit) {
        storage()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { content(main) } }
    }

    @Test
    fun withoutRepositoriesTheListOffersOnlyTheCheckAndItAnswersWithNothingNew() {
        var ran = 0
        shown { main -> RepoUpdates(main) { _, work -> ran++; main.act(false) { work() } } }
        compose.onNodeWithText(context.getString(R.string.repo_check)).assertIsDisplayed().performClick()
        compose.waitUntil(TIMEOUT_MS) { ran > 0 }
        compose.onNodeWithText(context.getString(R.string.repo_updates)).assertDoesNotExist()
        assertTrue(ran > 0)
    }

    @Test
    fun aPresetWithoutItsBaseInTheLibraryAsksForNothingWhenNoRepositoryHasIt() {
        shown { main -> Kin(main, "nowhere") {} }
        compose.onNodeWithText(context.getString(R.string.preset_install)).assertDoesNotExist()
    }

    private companion object {
        const val TIMEOUT_MS = 5_000L
    }
}
