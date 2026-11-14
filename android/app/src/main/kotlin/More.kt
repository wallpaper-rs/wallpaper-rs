package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun More(desk: Desk, open: (Aside) -> Unit, close: () -> Unit) {
    val status by watched(desk)
    val main = desk.main
    val run = desk.run
    var sheet by rememberSaveable { mutableStateOf<String?>(null) }
    var revoking by rememberSaveable { mutableStateOf<String?>(null) }
    var filesOpen by rememberSaveable { mutableStateOf(false) }
    val state = status.optString("state")
    val ready = status.optBoolean("encrypted") && state !in setOf("needs_key", "connected")
    TaskPage(null) {
        Header(stringResource(R.string.more_options), close)
        Group(stringResource(R.string.devices_group))
        Lead(Ph.DeviceMobile, stringResource(R.string.this_phone_row), stringResource(R.string.connected_now))
        others(status).forEach { device ->
            val name = device.text("label") ?: device.getString("name")
            val seen = device.optLong("seen_at").takeIf { it > 0 }?.let { stringResource(R.string.last_used, ago(it)) }
            Lead(Ph.Laptop, name, seen ?: device.optString("platform"), Modifier.testTag("device")) { revoking = device.getString("id") }
        }
        Setting(stringResource(R.string.approve_device), onClick = { sheet = "approve" })
        Setting(stringResource(R.string.let_other_sync), onClick = { sheet = "share" })
        Group(stringResource(R.string.sync_group))
        Toggle(stringResource(R.string.pause_sync), status.optBoolean("paused")) { run("") { account("pause", "paused" to it) } }
        Toggle(stringResource(R.string.listen_shown), status.optString("listen") == Heard.SHOWN) {
            run("") { withContext(Dispatchers.IO) { Native.setting(Heard.SETTING, if (it) Heard.SHOWN else "app") } }
        }
        Note(stringResource(R.string.listen_shown_note))
        Group(stringResource(R.string.cipher_group))
        Toggle(stringResource(R.string.private_share), status.optBoolean("encrypted") || (state == "connected" && !desk.plain), enabled = state == "connected") {
            desk.plain = !it
        }
        Toggle(stringResource(R.string.allow_plain), desk.plain, enabled = state == "connected") { desk.plain = it }
        Reason(stringResource(R.string.names_coded), Modifier.padding(horizontal = Measure.margin))
        if (status.optBoolean("encrypted")) {
            Group(stringResource(R.string.key_group))
            Setting(stringResource(R.string.recovery_title), sub = stringResource(R.string.recovery_sub), onClick = { sheet = "recovery" })
            if (state == "ready") Setting(stringResource(R.string.rotate_key), sub = stringResource(R.string.rotate_sub), onClick = { run(main.getString(R.string.rotating)) { account("rotate") } })
        }
        Group(stringResource(R.string.server_group))
        if (ready) {
            Setting(stringResource(R.string.private_files), sub = stringResource(R.string.private_sub), onClick = { filesOpen = !filesOpen })
            if (filesOpen) PrivateFiles(main, status, run)
            Setting(stringResource(R.string.fetch_all), sub = stringResource(R.string.fetch_all_sub), onClick = { fetchAll(desk) })
        }
        Setting(stringResource(R.string.print_row), sub = status.text("pin")?.let { it.take(8) + "…" } ?: stringResource(R.string.print_sub), onClick = { open(Aside.Print) })
        Setting(stringResource(R.string.remove_server_row), Modifier.testTag("remove-server"), onClick = { sheet = "remove" })
    }
    when (sheet) {
        "approve" -> Approve(main, run) { sheet = null }
        "share" -> Sheet({ sheet = null }, stringResource(R.string.let_other_sync)) {
            others(status).forEach { device ->
                val name = device.text("label") ?: device.getString("name")
                val waiting = stringResource(R.string.waiting_share, name)
                SheetOption(name, ph = Ph.Laptop, sub = device.optString("platform")) {
                    sheet = null
                    run(waiting) { account("share", "device" to device.getString("id")) }
                }
            }
        }
        "recovery" -> Sheet({ sheet = null }, stringResource(R.string.recovery_title)) { Recovery(status, run) }
        "remove" -> Question(
            stringResource(R.string.remove_server_question),
            stringResource(R.string.remove_server_note),
            stringResource(R.string.remove),
            stringResource(R.string.cancel),
            true,
            { sheet = null },
        ) {
            sheet = null
            run("") { account("remove") }
            close()
        }
    }
    revoking?.let { id ->
        val device = others(status).firstOrNull { it.getString("id") == id } ?: return@let
        Revoke(main, run, id, device.text("label") ?: device.getString("name")) { revoking = null }
    }
}

private fun fetchAll(desk: Desk) = desk.run(desk.main.getString(R.string.fetching_all)) {
    val files = (account("files") as? JSONObject)?.optJSONArray("files") ?: JSONArray()
    List(files.length(), files::getJSONObject).filter { it.isNull("item") && it.optBoolean("stored") }.forEach {
        account("fetch", "id" to it.getString("id"))
    }
}

@Composable
private fun Lead(ph: Ph, label: String, sub: String?, modifier: Modifier = Modifier, onClick: (() -> Unit)? = null) =
    Row(modifier.fillMaxWidth(), Arrangement.Start, Alignment.CenterVertically) {
        Glyph(ph, null, Modifier.padding(start = Measure.margin), tones.mist, size = 22.dp)
        Setting(label, Modifier.weight(1f), sub = sub, onClick = onClick)
    }
