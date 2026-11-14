package io.github.wallpaper_rs

import android.content.ClipboardManager
import android.content.Context
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import org.json.JSONObject

fun Context.pasted(): String =
    getSystemService(ClipboardManager::class.java)?.primaryClip?.takeIf { it.itemCount > 0 }?.getItemAt(0)?.coerceToText(this)?.toString()?.trim().orEmpty()

@Composable
fun Connect(desk: Desk, open: (Aside) -> Unit, done: () -> Unit, close: () -> Unit) {
    val status by watched(desk)
    val state = status.optString("state")
    val phone = stringResource(R.string.this_phone)
    LaunchedEffect(state, desk.task) {
        if (desk.task == null && state !in setOf("none", "linking", "unlinked")) done()
    }
    val link = status.optJSONObject("link")
    val failed = desk.failed
    TaskPage(foot = when {
        link != null || (desk.task != null && failed == null) -> null
        failed?.id == Desk.OTHER_CERTIFICATE && desk.blind == null -> {
            {
                QuietAction({ open(Aside.Print) }) { Glyph(Ph.ClipboardText, null); Text(stringResource(R.string.paste_print)) }
                Action({ desk.failed = null; open(Aside.Scan) }, Modifier.weight(1f).testTag("scan-again")) { Glyph(Ph.Scan, null); Text(stringResource(R.string.scan_again)) }
            }
        }
        failed?.id == Desk.OTHER_CERTIFICATE -> {
            {
                QuietAction({ desk.failed = null }) { Text(stringResource(R.string.change_address)) }
                Action({ open(Aside.Print) }, Modifier.weight(1f)) { Glyph(Ph.ClipboardText, null); Text(stringResource(R.string.paste_print)) }
            }
        }
        failed?.id == Desk.EXPIRED -> {
            {
                QuietAction({ desk.failed = null; close() }) { Text(stringResource(R.string.cancel)) }
                Action({ desk.connect(phone) }, Modifier.weight(1f)) { Glyph(Ph.ArrowClockwise, null); Text(stringResource(R.string.new_code)) }
            }
        }
        failed != null -> {
            {
                QuietAction({ desk.failed = null }) { Text(stringResource(R.string.change_address)) }
                Action({ desk.connect(phone) }, Modifier.weight(1f)) { Glyph(Ph.ArrowClockwise, null); Text(stringResource(R.string.retry)) }
            }
        }
        else -> {
            {
                Action({ desk.connect(phone) }, Modifier.weight(1f).testTag("connect"), desk.url.isNotBlank() || state != "none") {
                    Glyph(Ph.LinkSimple, null)
                    Text(stringResource(R.string.connect))
                }
            }
        }
    }) {
        Header(stringResource(R.string.connect_block), close)
        when {
            link != null -> Waiting(desk, status, link)
            failed != null -> Failed(desk, status, failed)
            desk.task != null -> Progress(stringResource(R.string.checking_server), null)
            else -> Form(desk, status, open)
        }
    }
    desk.asked?.let { seen ->
        Question(
            stringResource(R.string.cert_title),
            stringResource(R.string.cert_question, seen.chunked(4).joinToString(" ")),
            stringResource(R.string.cert_trust),
            stringResource(R.string.cancel),
            false,
            { desk.asked = null },
        ) {
            desk.asked = null
            desk.print = seen
            desk.connect(phone)
        }
    }
}

@Composable
private fun Form(desk: Desk, status: JSONObject, open: (Aside) -> Unit) {
    val context = LocalContext.current
    Text(stringResource(R.string.connect_intro), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), tones.mist, style = Type.text)
    if (status.optString("state") == "none") {
        TextBox(
            stringResource(R.string.server_address),
            desk.url,
            { desk.url = it; desk.print = null; desk.scanned = false },
            Modifier.testTag("address"),
            example = stringResource(R.string.server_example),
        ) {
            Square({ context.pasted().takeIf { it.isNotEmpty() }?.let { desk.url = it; desk.print = null } }, Ph.ClipboardText, stringResource(R.string.paste), boxed = false)
            if (desk.blind == null) Square({ open(Aside.Scan) }, Ph.Scan, stringResource(R.string.scan_code), Modifier.testTag("scan"), boxed = false)
        }
        desk.blind?.let { Text(stringResource(it), Modifier.padding(horizontal = Measure.margin).testTag("typing-stays"), tones.mist, style = Type.foot) }
    } else {
        Found(status.server(), host(status.optString("url")))
    }
    TextBox(stringResource(R.string.phone_label), desk.label, { desk.label = it }, kind = KeyboardType.Text, example = stringResource(R.string.this_phone))
    Nearby { desk.url = it; desk.print = null }
}

@Composable
private fun Waiting(desk: Desk, status: JSONObject, link: JSONObject) {
    val shown = status.optString("url").ifEmpty { desk.url }
    Found(host(shown), null)
    if (desk.scanned) StateLine(stringResource(R.string.checked_by_code), Tone.Sprout, Modifier.padding(horizontal = Measure.margin))
    Block(stringResource(R.string.approve_code), null)
    Digits(link.getString("code"), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e4).testTag("link-code"))
    Text(stringResource(R.string.approve_where), Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text)
    Progress(stringResource(R.string.waiting_approval), null, Modifier.padding(top = Measure.e5))
    WordAction({ desk.main.act(false) { account("cancel") } }, Modifier.padding(horizontal = Measure.e2)) { Text(stringResource(R.string.cancel)) }
}

@Composable
private fun Failed(desk: Desk, status: JSONObject, failed: Refused) {
    val main = desk.main
    val where = host(desk.url.ifBlank { status.optString("url") })
    when (failed.id) {
        Desk.OTHER_CERTIFICATE -> Problem(stringResource(R.string.print_mismatch), stringResource(R.string.print_mismatch_text, where), Modifier.testTag("problem"), main.told(failed))
        Desk.EXPIRED -> Problem(stringResource(R.string.code_expired), stringResource(R.string.code_expired_text), Modifier.testTag("problem"), main.told(failed))
        Desk.UNREACHABLE -> Problem(stringResource(R.string.cannot_reach, where), stringResource(R.string.cannot_reach_hint), Modifier.testTag("problem"), main.told(failed))
        else -> Problem(main.told(failed).ifBlank { stringResource(R.string.failed_without_reason) }, null, Modifier.testTag("problem"))
    }
}
