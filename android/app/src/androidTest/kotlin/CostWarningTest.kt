package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Text
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONObject
import org.junit.After
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class CostWarningTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun shown(report: String) {
        storage()
        val parsed = JSONObject(report)
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent {
                Column {
                    CostWarning(parsed)
                    frameNote(parsed)?.let { Text(it) }
                }
            }
        }
    }

    private fun told(id: Int, vararg args: Any) =
        compose.onNodeWithText(context.getString(id, *args), substring = true).assertIsDisplayed()

    @Test
    fun aStillSceneTellsWhyUnderTheQualitySetting() {
        shown("""{ "auto": "still", "over": null, "slow": null, "left_out": [], "step": "still", "frame_ms": 70.0, "period_ms": 16.7 }""")
        told(R.string.level_stopped)
        told(R.string.frame_measured, "70.0", "16.7", context.getString(R.string.step_still))
    }

    @Test
    fun aFixedLevelMeasuredTooSlowTellsItsFrameTimeInsteadOfTheEstimate() {
        shown("""{ "auto": "lower", "over": null, "slow": [25.0, 16.7], "left_out": [], "step": "steady", "frame_ms": 25.0, "period_ms": 16.7 }""")
        told(R.string.frames_slow, "25.0", "16.7")
        told(R.string.auto_would_lower)
        compose.onNodeWithText(context.getString(R.string.level_over, "6.0", "4"), substring = true).assertDoesNotExist()
    }

    @Test
    fun aFixedLevelOverTheEstimateBeforeAnyFrameTellsItsCost() {
        shown("""{ "auto": "lower", "over": [6.0, 4.0], "slow": null, "left_out": [], "step": "model", "frame_ms": null, "period_ms": 16.7 }""")
        told(R.string.level_over, "6.0", "4")
        told(R.string.frame_unmeasured, context.getString(R.string.step_model))
    }

    @Test
    fun framesThatFitShowNoWarning() {
        shown("""{ "auto": "", "over": null, "slow": null, "left_out": [], "step": "steady", "frame_ms": 5.0, "period_ms": 16.7 }""")
        compose.onNodeWithText(context.getString(R.string.auto_would_lower), substring = true).assertDoesNotExist()
        told(R.string.frame_measured, "5.0", "16.7", context.getString(R.string.step_steady))
    }
}
