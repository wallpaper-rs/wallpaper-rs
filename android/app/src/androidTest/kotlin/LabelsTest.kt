package io.github.wallpaper_rs

import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.runtime.getValue
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithContentDescription
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.performTextReplacement
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class LabelsTest {
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

    private fun item() = runBlocking { Item(ask("item", "uid" to uid) as JSONObject) }

    @Test
    fun starsTagsAndTheTitleOfAPictureAreSetFromItsPage() {
        val file = still(0xff336699.toInt())
        uid = ParcelFileDescriptor.open(file, ParcelFileDescriptor.MODE_READ_ONLY).use { JSONObject(Native.store(it.fd, "Evening.png")!!).getString("uid") }
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent {
                val shown by loaded<Item?>(null, main.revision.intValue) { Item(ask("item", "uid" to uid) as JSONObject) }
                shown?.let { Labels(main, it) }
            }
        }
        compose.onNodeWithContentDescription(context.getString(R.string.rate_stars, 4)).performClick()
        assertTrue(waitFor { item().rating == 4 })
        compose.onNodeWithContentDescription(context.getString(R.string.rate_stars, 4)).performClick()
        assertTrue("the same star again clears the rating", waitFor { item().rating == 0 })
        compose.onNodeWithText(context.getString(R.string.tag_add)).performClick()
        compose.onNodeWithText(context.getString(R.string.tag_name)).performTextInput("calm, blue")
        compose.onNodeWithText(context.getString(R.string.save)).performClick()
        assertTrue(waitFor { item().tags.sorted() == listOf("blue", "calm") })
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithContentDescription(context.getString(R.string.tag_remove, "calm")).fetchSemanticsNodes().isNotEmpty() }
        compose.onNodeWithContentDescription(context.getString(R.string.tag_remove, "calm")).assertIsDisplayed().performClick()
        assertTrue(waitFor { item().tags == listOf("blue") })
        compose.onNodeWithText(context.getString(R.string.rename)).performClick()
        compose.onNodeWithText("Evening").performTextReplacement("Evening glow")
        compose.onNodeWithText(context.getString(R.string.save)).performClick()
        assertTrue(waitFor { item().title == "Evening glow" })
        assertEquals(0, item().rating)
    }

    private companion object {
        const val TIMEOUT_MS = 5_000L
    }
}
