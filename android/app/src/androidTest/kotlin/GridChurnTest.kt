package io.github.wallpaper_rs

import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.material3.Text
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.test.core.app.ActivityScenario
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class GridChurnTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private var scenario: ActivityScenario<Main>? = null
    private val made = mutableListOf<String>()

    @After
    fun closed() {
        scenario?.close()
        runBlocking { ask("remove", "uids" to made.toList()) }
    }

    private fun stored(index: Int): String = ParcelFileDescriptor.open(still(0xff203040.toInt() + index * 0x010101), ParcelFileDescriptor.MODE_READ_ONLY)
        .use { JSONObject(Native.store(it.fd, "Churn $index.png")!!).getString("uid") }

    @Test
    fun theLibraryGridSurvivesAHundredChangesWhileItIsDrawn() {
        scenario = ActivityScenario.launch(Main::class.java)
        repeat(CHANGES) { index ->
            if (index % 3 == 2 && made.isNotEmpty()) runBlocking { ask("remove", "uids" to listOf(made.removeAt(0))) } else made += stored(index)
            scenario?.onActivity { it.revision.intValue++ }
            if (index % 10 == 0) compose.waitForIdle()
        }
        compose.waitForIdle()
        repeat(SWAPS) { index ->
            scenario?.close()
            val churn = Thread { repeat(SWAPS) { made += stored(CHANGES + index * SWAPS + it) } }.apply { start() }
            scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
                main.revision.intValue++
                main.setContent { Text("swap $index") }
            }
            churn.join()
        }
        compose.waitForIdle()
    }

    private companion object {
        const val CHANGES = 100
        const val SWAPS = 10
    }
}
