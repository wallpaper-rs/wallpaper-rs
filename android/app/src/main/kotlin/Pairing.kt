package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import java.text.DateFormat
import java.util.Date
import org.json.JSONObject

@Composable
fun Pairing(main: Main, status: JSONObject, run: Run) {
    val pairing = status.optJSONObject("pairing") ?: return
    val (name, sharing) = pairing.getString("name") to pairing.getBoolean("sharing")
    val code = pairing.getString("code")
    val waiting = if (sharing) "" else main.getString(R.string.waiting_key, name)
    val done = stringResource(if (sharing) R.string.key_sent else R.string.key_received, name)
    val cancel = { main.act(false) { account("cancel") } }
    Sheet({ cancel() }) {
        Title(stringResource(R.string.compare_screens))
        Digits(code, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e3).testTag("digits"))
        Text(stringResource(if (sharing) R.string.compare_here else R.string.compare_there, name), Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text)
        Row(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e4), Arrangement.spacedBy(Measure.e2)) {
            QuietAction({ cancel() }, Modifier.weight(1f)) { Text(stringResource(R.string.cancel)) }
            Action({
                run(waiting) {
                    account("confirm", "code" to code)
                    main.toast(done)
                }
            }, Modifier.weight(1f)) { Text(stringResource(R.string.they_match)) }
        }
    }
}

@Composable
private fun Title(text: String) =
    Text(text, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), style = Type.row.copy(fontFamily = RedHat, fontWeight = FontWeight.Bold))

@Composable
fun Approve(main: Main, run: Run, close: () -> Unit) = Sheet(close) {
    var code by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var found by remember { mutableStateOf<JSONObject?>(null) }
    val approved = stringResource(R.string.approved)
    Title(stringResource(R.string.approve_device))
    Text(stringResource(R.string.approve_warning), Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text)
    TextBox(stringResource(R.string.device_code), code, { if (found == null) code = it })
    found?.let { device ->
        val client = device.text("client")?.ifEmpty { null } ?: stringResource(R.string.unnamed)
        val time = DateFormat.getDateTimeInstance(DateFormat.SHORT, DateFormat.SHORT).format(Date(device.optLong("requested_at")))
        Text(stringResource(R.string.asked, device.optString("platform"), client, time), Modifier.padding(horizontal = Measure.margin), style = Type.text)
        TextBox(stringResource(R.string.account_password), password, { password = it }, hidden = true, example = stringResource(R.string.approve_password))
    }
    Row(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.margin, top = Measure.e4), Arrangement.spacedBy(Measure.e2)) {
        QuietAction(close, Modifier.weight(1f)) { Text(stringResource(R.string.cancel)) }
        if (found == null) {
            Action({ main.act(false) { found = account("preview", "code" to code) as? JSONObject } }, Modifier.weight(1f), code.isNotBlank()) {
                Text(stringResource(R.string.look_up))
            }
        } else {
            Action({
                close()
                run("") {
                    account("approve", "code" to code, "approve" to true, "password" to password)
                    main.toast(approved)
                }
            }, Modifier.weight(1f), password.isNotEmpty()) { Text(stringResource(R.string.approve)) }
        }
    }
}

@Composable
fun Revoke(main: Main, run: Run, id: String, name: String, close: () -> Unit) = Sheet(close) {
    var password by remember { mutableStateOf("") }
    val revoked = stringResource(R.string.revoked, name)
    Title(stringResource(R.string.stop_syncing_with, name))
    Text(stringResource(R.string.stop_syncing_note), Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text)
    TextBox(stringResource(R.string.your_account_password), password, { password = it }, hidden = true)
    Action({
        close()
        run("") {
            account("revoke", "device" to id, "password" to password)
            main.toast(revoked)
        }
    }, Modifier.fillMaxWidth().padding(horizontal = Measure.margin), password.isNotEmpty(), Kind.Remove) {
        Glyph(Ph.LinkBreak, null)
        Text(stringResource(R.string.stop_syncing))
    }
    WordAction(close, Modifier.fillMaxWidth().padding(horizontal = Measure.margin)) { Text(stringResource(R.string.cancel)) }
}
