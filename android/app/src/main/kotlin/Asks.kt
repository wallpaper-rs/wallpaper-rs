package io.github.wallpaper_rs

import android.Manifest.permission.RECORD_AUDIO
import android.content.pm.PackageManager.PERMISSION_GRANTED
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONArray

private val LABELS = mapOf(
    "audio" to R.string.react_sound, "audio_network" to R.string.audio_network_title, "touch" to R.string.react_touch,
    "taps" to R.string.react_taps, "media" to R.string.show_media, "media_network" to R.string.show_media_network,
    "network" to R.string.web_reach,
)

suspend fun asked(uid: String): List<String> = strings(ask("asks", "uid" to uid) as? JSONArray).filter { it in LABELS }

@Composable
fun Asks(main: Main, uid: String, undecided: List<String>, modifier: Modifier = Modifier) {
    if (undecided.isEmpty()) return
    val decide = deciding(main, uid)
    Column(modifier.fillMaxWidth().testTag("asks")) {
        Group(stringResource(R.string.asks_title))
        Text(stringResource(R.string.asks_note), Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.foot)
        undecided.forEach { Ask(it, decide) }
    }
}

@Composable
fun AsksSheet(main: Main, uid: String, close: () -> Unit) {
    val undecided by loaded<List<String>?>(null, uid, main.revision.intValue) { asked(uid) }
    val decide = deciding(main, uid)
    val shown = undecided ?: return
    if (shown.isEmpty()) {
        LaunchedEffect(Unit) { close() }
        return
    }
    Sheet(close, stringResource(R.string.asks_title), stringResource(R.string.asks_note)) {
        shown.forEach { Ask(it, decide) }
    }
}

@Composable
private fun deciding(main: Main, uid: String): (String, Boolean) -> Unit {
    val refused = stringResource(R.string.microphone_needed)
    val send = { input: String, allow: Boolean ->
        val (yes, no) = if (allow) listOf(input) to emptyList<String>() else emptyList<String>() to listOf(input)
        main.act(false) { ask("decide", "uid" to uid, "allow" to yes, "deny" to no) }
        Unit
    }
    val microphone = rememberLauncherForActivityResult(RequestPermission()) { if (it) send("audio", true) else main.toast(refused) }
    return { input, allow ->
        if (allow && input == "audio" && main.checkSelfPermission(RECORD_AUDIO) != PERMISSION_GRANTED) microphone.launch(RECORD_AUDIO) else send(input, allow)
    }
}

@Composable
private fun Ask(input: String, decide: (String, Boolean) -> Unit) =
    Column(Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e2).testTag("ask_$input")) {
        Text(stringResource(LABELS.getValue(input)), style = Type.row)
        Row(Modifier.fillMaxWidth().padding(top = Measure.e2), Arrangement.spacedBy(Measure.e2)) {
            QuietAction({ decide(input, false) }, Modifier.weight(1f)) { Text(stringResource(R.string.asks_without)) }
            Action({ decide(input, true) }, Modifier.weight(1f)) { Text(stringResource(R.string.asks_allow)) }
        }
    }
