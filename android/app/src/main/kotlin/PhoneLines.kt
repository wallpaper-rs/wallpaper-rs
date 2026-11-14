package io.github.wallpaper_rs

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONObject

private val NOTICES = listOf("onnxruntime", "onnxruntime-third-party-notices", "depth-anything-v2", "lama", "onest", "red-hat-display", "phosphor")

val PHONE = listOf(
    Line(R.string.mobile_data, Block.Phone) { k ->
        Switch(stringResource(R.string.mobile_data), k.on(Sync.NETWORK, "any"), stringResource(R.string.mobile_data_sub)) {
            k.set(Sync.NETWORK, if (it) "any" else "unmetered")
        }
    },
    Line(R.string.allow_download, Block.Phone) { k ->
        Switch(stringResource(R.string.allow_download), k.on("models.allow_download", "true"), stringResource(R.string.allow_download_sub)) {
            k.set("models.allow_download", if (it) "true" else "false")
        }
    },
    Line(R.string.depth_model_name, Block.Phone, always = false) { DepthModel(it) },
    Line(R.string.lama_model_name, Block.Phone, always = false) { LamaNote(it.main, it["parallax.fill"], it.changes) },
    Line(R.string.remove_wallpaper, Block.Phone) { Removal(it.main) },
    Line(R.string.shortcut_missed_name, Block.Phone, always = false) { ShortcutMissed() },
    Line(R.string.diagnostics, Block.Phone) { Diagnostics(it) },
    Line(R.string.licenses, Block.Phone) { Licenses() },
)

@Composable
private fun DepthModel(k: Knobs) {
    val model by loaded(JSONObject(), k.changes, Depth.working.value) { depth("model") as JSONObject }
    if (model.optBoolean("installed")) Setting(stringResource(R.string.depth_model, model.getString("size")), trailing = {
        WordAction({ k.change { depth("forget") } }, enabled = Depth.working.value == null) { Text(stringResource(R.string.delete_model)) }
    })
}

@Composable
private fun Diagnostics(k: Knobs) {
    var open by rememberSaveable { mutableStateOf(false) }
    Fold(stringResource(R.string.diagnostics), open) { open = !open }
    if (!open) return
    reportLines(k.report).forEach { Text(it, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e1), tones.mist, style = Type.foot) }
    if ((k.report.optJSONArray("left_out")?.length() ?: 0) > 0) {
        WordAction({ k.change { Native.retry() } }, Modifier.padding(horizontal = Measure.e2)) { Text(stringResource(R.string.retry_effects)) }
    }
    PerfControls()
}

@Composable
private fun Licenses() {
    var open by rememberSaveable { mutableStateOf(false) }
    var notice by rememberSaveable { mutableStateOf<String?>(null) }
    Fold(stringResource(R.string.licenses), open) { open = !open }
    if (open) NOTICES.forEach { Setting(it, onClick = { notice = it }) }
    notice?.let { Notice(it) { notice = null } }
}

@Composable
private fun Notice(name: String, close: () -> Unit) {
    val assets = LocalContext.current.assets
    val lines by loaded(emptyList<String>(), name) { assets.open("$name.txt").bufferedReader().use { it.readLines() } }
    Sheet(close, name) {
        LazyColumn(Modifier.heightIn(max = NOTICE_HEIGHT)) {
            items(lines) { Text(it, Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.foot) }
        }
        WordAction(close, Modifier.padding(horizontal = Measure.e2)) { Text(stringResource(R.string.close)) }
    }
}

private val NOTICE_HEIGHT = 420.dp
