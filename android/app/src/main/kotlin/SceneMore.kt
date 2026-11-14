package io.github.wallpaper_rs

import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.CreateDocument
import androidx.activity.result.contract.ActivityResultContracts.OpenDocument
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun SceneMore(main: Main, uid: String, form: JSONObject, modifier: Modifier = Modifier) {
    val context = LocalContext.current
    val skipped = stringResource(R.string.skipped_settings)
    var naming by remember(uid) { mutableStateOf<Pair<String, String>?>(null) }
    var exporting by remember(uid) { mutableStateOf("") }
    var imported by remember(uid) { mutableStateOf<String?>(null) }
    val writing = rememberLauncherForActivityResult(CreateDocument(JSON)) { target ->
        target?.let { main.act(false) {
            val text = ask("export", "uid" to uid, "preset" to exporting) as String
            withContext(Dispatchers.IO) { context.contentResolver.openOutputStream(it)?.use { out -> out.write(text.toByteArray()) } }
        } }
    }
    val read: suspend (Uri) -> String = { source ->
        withContext(Dispatchers.IO) { context.contentResolver.openInputStream(source)?.use { it.readBytes().decodeToString() }.orEmpty() }
    }
    val bring: suspend (String, String) -> Unit = { text, name ->
        val warnings = ask("import", "uid" to uid, "text" to text, "name" to name) as JSONArray
        if (warnings.length() > 0) main.toast(skipped.format((0 until warnings.length()).joinToString { warnings.getString(it) }))
    }
    val reading = rememberLauncherForActivityResult(OpenDocument()) { source -> source?.let { main.act { bring(read(it), "") } } }
    val keeping = rememberLauncherForActivityResult(OpenDocument()) { source -> source?.let { main.act(false) { imported = read(it) } } }
    Column(modifier) {
        if (form.optBoolean("sounds") && form.optBoolean("muted")) Text(stringResource(R.string.sounds_muted), Modifier.padding(16.dp, 4.dp), style = MaterialTheme.typography.bodySmall)
        val saved = options(form.optJSONArray("saved"))
        val presets = options(form.optJSONArray("presets")) + saved
        if (presets.isNotEmpty()) Choices(stringResource(R.string.presets), presets, "") { preset ->
            main.act { ask("preset", "uid" to uid, "preset" to preset) }
        }
        saved.forEach { (preset, name) ->
            Saved(name, { naming = preset to name }, { exporting = preset; writing.launch("$name.json") }) { main.act { ask("forget", "preset" to preset) } }
        }
        FlowRow(Modifier.padding(horizontal = 8.dp)) {
            WordAction({ naming = "" to "" }) { Text(stringResource(R.string.save_preset)) }
            WordAction({ exporting = ""; writing.launch(FILE) }) { Text(stringResource(R.string.export_settings)) }
            WordAction({ reading.launch(arrayOf(JSON, "text/plain")) }) { Text(stringResource(R.string.import_settings)) }
            WordAction({ keeping.launch(arrayOf(JSON, "text/plain")) }) { Text(stringResource(R.string.import_preset)) }
        }
        if ((form.optJSONArray("fields")?.length() ?: 0) > 0) WordAction({ main.act { ask("reset", "uid" to uid) } }, Modifier.padding(horizontal = 8.dp)) {
            Text(stringResource(R.string.reset))
        }
        WebPrivacy(main, uid, form.optJSONObject("web"))
    }
    imported?.let { text ->
        Naming("", { imported = null }) { chosen ->
            imported = null
            main.act { bring(text, chosen) }
        }
    }
    naming?.let { (preset, name) ->
        Naming(name, { naming = null }) { chosen ->
            naming = null
            main.act { if (preset.isEmpty()) ask("save", "uid" to uid, "name" to chosen) else ask("rename", "preset" to preset, "name" to chosen) }
        }
    }
}

private const val JSON = "application/json"
private const val FILE = "settings.json"
