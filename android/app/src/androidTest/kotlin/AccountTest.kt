package io.github.wallpaper_rs

import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class AccountTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun shown(status: JSONObject, prepare: (Desk) -> Unit = {}, body: @Composable (Desk) -> Unit): Desk {
        storage()
        var made: Desk? = null
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            val desk = Desk(main).apply { this.status = { status } }
            prepare(desk)
            made = desk
            main.setContent { Column { body(desk) } }
        }
        compose.waitForIdle()
        return made!!
    }

    private fun text(id: Int, vararg args: Any) = compose.onNodeWithText(context.getString(id, *args))

    private fun account(status: JSONObject, prepare: (Desk) -> Unit = {}) = shown(status, prepare) { Account(it, {}, {}) {} }

    @Test
    fun withoutAServerThePageSaysSoAndOffersOnlyConnect() {
        account(JSONObject(mapOf("state" to "none")))
        text(R.string.not_connected).assertIsDisplayed()
        text(R.string.connect_block).assertIsDisplayed()
        text(R.string.connect).assertIsDisplayed()
        compose.onNodeWithTag("more").assertDoesNotExist()
    }

    @Test
    fun connectingAsksForTheAddressAndThenShowsTheCodeToApprove() {
        val status = JSONObject(mapOf("state" to "none"))
        shown(status) { Connect(it, {}, {}) {} }
        compose.onNodeWithTag("address").assertIsDisplayed()
        compose.onNodeWithTag("connect").assertIsDisplayed()
        status.put("state", "linking").put("url", URL).put("link", JSONObject(mapOf("code" to "ABCD-EFGH")))
        scenario?.onActivity { main -> main.setContent { Column { Connect(Desk(main).apply { this.status = { status } }, {}, {}) {} } } }
        compose.onNodeWithTag("link-code").assertIsDisplayed()
        text(R.string.waiting_approval).assertIsDisplayed()
    }

    @Test
    fun aSyncedAccountShowsWhereWhenAndThatItIsEncrypted() {
        account(synced())
        text(R.string.connected_to, "casa").assertIsDisplayed()
        compose.onNode(hasText(context.getString(R.string.sealed), substring = true)).assertIsDisplayed()
        text(R.string.sync_now).assertIsDisplayed()
        compose.onNodeWithTag("more").assertIsDisplayed()
    }

    @Test
    fun theRareThingsWaitBehindMoreOptions() {
        shown(synced()) { More(it, {}) {} }
        text(R.string.this_phone_row).assertExists()
        text(R.string.cipher_group).assertExists()
        text(R.string.print_row).assertExists()
        text(R.string.remove_server_row).assertExists()
    }

    @Test
    fun aServerOutOfReachSaysSoWithRetryAtTheFoot() {
        account(synced()) { it.failed = Refused("cannot reach", Desk.UNREACHABLE, listOf("casa.example")) }
        text(R.string.offline_from, "casa").assertIsDisplayed()
        text(R.string.cannot_reach, "casa.example").assertIsDisplayed()
        text(R.string.retry).assertIsDisplayed()
    }

    @Test
    fun connectingOnlyOffersToSyncAndNotNowLeavesItOff() {
        val desk = account(JSONObject(mapOf("state" to "connected", "url" to URL, "name" to "casa"))) { it.offer = true }
        text(R.string.not_synced_yet).assertIsDisplayed()
        compose.onNodeWithTag("offer").assertIsDisplayed()
        text(R.string.not_now).performClick()
        compose.onNodeWithTag("offer").assertDoesNotExist()
        assertEquals(null, desk.task)
        assertEquals("connected", desk.status().optString("state"))
    }

    private fun synced() = JSONObject(
        mapOf("state" to "ready", "url" to URL, "name" to "casa", "encrypted" to true, "last_sync" to System.currentTimeMillis() / 1000 - 180),
    )

    private companion object {
        const val URL = "https://casa.example"
    }
}
