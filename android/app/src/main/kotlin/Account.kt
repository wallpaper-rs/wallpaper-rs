package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONObject

private val ARRIVED = 132.dp
private const val COVER = 0.75f
private const val SHOWN_ARRIVALS = 6

@Composable
fun Account(desk: Desk, open: (Aside) -> Unit, library: () -> Unit, close: () -> Unit) {
    val status by watched(desk)
    val state = status.optString("state")
    val main = desk.main
    val offline = desk.failed?.id in Desk.OFFLINE || status.optJSONObject("last_reply")?.optString("id") in Desk.OFFLINE
    val linked = state !in setOf("none", "linking", "unlinked")
    val arrived = desk.arrived
    TaskPage(foot = {
        when {
            !linked -> Action({ open(Aside.Connect) }, Modifier.weight(1f)) { Glyph(Ph.LinkSimple, null); Text(stringResource(R.string.connect)) }
            desk.task != null -> {}
            arrived != null -> Action({ desk.arrived = null; library() }, Modifier.weight(1f)) { Glyph(Ph.Images, null); Text(stringResource(R.string.see_in_library)) }
            offline -> Action(desk::sync, Modifier.weight(1f)) { Glyph(Ph.ArrowClockwise, null); Text(stringResource(R.string.retry)) }
            state == "signed_out" -> Action({ desk.connect(main.getString(R.string.this_phone)) }, Modifier.weight(1f)) { Glyph(Ph.LinkSimple, null); Text(stringResource(R.string.connect)) }
            else -> Action({ start(desk, status) }, Modifier.weight(1f)) { Glyph(Ph.ArrowsClockwise, null); Text(stringResource(R.string.sync_now)) }
        }
    }, above = { if (desk.offer && state == "connected" && desk.task == null) Offer(status.server(), desk) }) {
        Header(stringResource(R.string.account), close)
        if (!linked) return@TaskPage Unlinked()
        val server = status.server()
        StripHead(stringResource(if (offline) R.string.offline_from else R.string.connected_to, server)) { Line(desk, status, offline) }
        Setting(stringResource(R.string.server_group), sub = host(status.optString("url")))
        when {
            offline -> Problem(
                stringResource(R.string.cannot_reach, host(status.optString("url"))),
                stringResource(R.string.cannot_reach_hint),
                details = desk.failed?.let(main::told) ?: status.text("last_error"),
            )
            desk.failed != null -> Problem(main.told(desk.failed!!), null)
            state == "signed_out" -> Problem(stringResource(R.string.signed_out), null)
            status.optBoolean("outdated") -> Problem(stringResource(R.string.outdated), null)
        }
        when {
            desk.task != null && linked -> Block(stringResource(R.string.sync_block), stringResource(R.string.receiving)) {
                Progress(stringResource(R.string.syncing), null, Modifier.padding(0.dp)) { main.act(false) { account("cancel") } }
            }
            arrived != null -> Arrived(arrived)
            else -> Block(stringResource(R.string.sync_block), stringResource(if (state == "connected") R.string.sync_explain else R.string.sync_digits))
        }
        Setting(stringResource(R.string.more_options), Modifier.testTag("more"), onClick = { open(Aside.More) })
    }
    Pairing(main, status, desk.run)
    Chooser(desk, status)
}

private fun start(desk: Desk, status: JSONObject) = when (status.optString("state")) {
    "connected" -> desk.syncing()
    "needs_key" -> others(status).singleOrNull()?.let { join(desk, it) } ?: run { desk.choosing = true }
    else -> desk.sync()
}

fun join(desk: Desk, device: JSONObject) {
    val name = device.text("label") ?: device.getString("name")
    desk.run(desk.main.getString(R.string.waiting_join, name)) { account("join", "device" to device.getString("id")) }
}

@Composable
private fun Unlinked() {
    StripHead(stringResource(R.string.not_connected)) { Text(stringResource(R.string.only_here), color = tones.frost, style = Type.text) }
    Block(stringResource(R.string.connect_block), stringResource(R.string.connect_explain))
    ThinSeam()
    Block(stringResource(R.string.sync_block), stringResource(R.string.connect_first)) {
        QuietAction({}, enabled = false) { Glyph(Ph.ArrowsClockwise, null); Text(stringResource(R.string.sync_now)) }
    }
}

@Composable
private fun Line(desk: Desk, status: JSONObject, offline: Boolean) {
    val state = status.optString("state")
    val last = status.optLong("last_sync")
    val sealed = stringResource(if (status.optBoolean("encrypted")) R.string.sealed else R.string.unsealed)
    when {
        offline -> Text(stringResource(R.string.still_here), style = Type.text)
        desk.task != null -> StateText(stringResource(R.string.syncing), Ph.ArrowsClockwise)
        state == "connected" -> Text(stringResource(R.string.not_synced_yet), color = tones.mist, style = Type.text)
        state == "needs_key" -> Text(stringResource(R.string.connected_not_synced), color = tones.mist, style = Type.text)
        state == "paused" -> StateLine(stringResource(R.string.sync_paused), Tone.Amber)
        last > 0 -> StateText(stringResource(R.string.synced_ago, ago(last), sealed), Ph.CloudCheck, tones.sprout)
        else -> Text(stringResource(R.string.not_synced_yet), color = tones.mist, style = Type.text)
    }
}

@Composable
private fun StateText(text: String, ph: Ph, ink: androidx.compose.ui.graphics.Color = tones.frost) =
    Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
        Glyph(ph, null, Modifier.padding(top = 2.dp), tint = ink, size = 16.dp)
        Text(text, color = ink, style = Type.text.copy(fontWeight = Type.row.fontWeight))
    }

@Composable
private fun Arrived(items: List<Item>) = Column(Modifier.padding(top = Measure.e5).testTag("arrived")) {
    Row(Modifier.padding(horizontal = Measure.margin), Arrangement.spacedBy(10.dp)) {
        Glyph(Ph.CheckCircle, null, Modifier.padding(top = 2.dp), tint = tones.sprout, lit = true)
        Text(stringResource(R.string.arrived, items.size), style = Type.row.copy(fontFamily = RedHat, fontWeight = Type.titleS.fontWeight))
    }
    Text(stringResource(R.string.arrived_note), Modifier.padding(start = Measure.margin + 31.dp, end = Measure.margin, top = 6.dp), tones.mist, style = Type.text)
    Row(Modifier.horizontalScroll(rememberScrollState()).padding(Measure.margin, Measure.e4), Arrangement.spacedBy(Measure.e3)) {
        items.take(SHOWN_ARRIVALS).forEach { item ->
            Column(Modifier.width(ARRIVED)) {
                Thumb(item, Modifier.aspectRatio(COVER))
                Text(item.title, Modifier.padding(top = Measure.e1), tones.mist, maxLines = 1, style = Type.foot)
            }
        }
    }
}

@Composable
private fun Offer(server: String, desk: Desk) {
    val shape = RoundedCornerShape(Measure.rNotice)
    Column(
        Modifier.padding(Measure.e3).fillMaxWidth().background(tones.nightHigh, shape).border(Measure.line, tones.nightLine, shape)
            .padding(start = Measure.e4, end = Measure.e2, top = Measure.e3, bottom = Measure.e1).testTag("offer"),
    ) {
        Row(horizontalArrangement = Arrangement.spacedBy(Measure.e3)) {
            Glyph(Ph.CloudCheck, null, size = Measure.iconSquare)
            Text(stringResource(R.string.sync_offer, server), Modifier.weight(1f), style = Type.text)
        }
        Row(Modifier.fillMaxWidth(), Arrangement.End) {
            WordAction({ desk.offer = false }) { Text(stringResource(R.string.not_now)) }
            WordAction(desk::syncing) { Text(stringResource(R.string.sync_short)) }
        }
    }
}

@Composable
private fun Chooser(desk: Desk, status: JSONObject) {
    if (!desk.choosing) return
    Sheet({ desk.choosing = false }, stringResource(R.string.sync_with_which)) {
        others(status).forEach { device ->
            SheetOption(device.text("label") ?: device.getString("name"), ph = Ph.Laptop, sub = device.optString("platform")) {
                desk.choosing = false
                join(desk, device)
            }
        }
    }
}
