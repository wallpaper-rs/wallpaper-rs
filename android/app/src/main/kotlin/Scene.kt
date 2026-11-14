package io.github.wallpaper_rs

import android.Manifest.permission.RECORD_AUDIO
import android.content.pm.PackageManager.PERMISSION_GRANTED
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalLocale
import androidx.compose.ui.res.stringResource
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun sceneForm(main: Main, uid: String): State<JSONObject> {
    val languages = listOf(LocalLocale.current.toLanguageTag().replace('-', '_'))
    return loaded(JSONObject(), uid, main.revision.intValue) { ask("scene", "uid" to uid, "languages" to languages) as JSONObject }
}

@Composable
fun SceneControls(main: Main, uid: String, form: JSONObject, modifier: Modifier = Modifier) {
    val refused = stringResource(R.string.microphone_needed)
    var flipped by remember(uid) { mutableStateOf(setOf<String>()) }
    val set: (String, Any?) -> Unit = { key, value -> main.act { ask("property", "uid" to uid, "key" to key, "value" to value) } }
    val grant = { input: String, on: Boolean -> main.act(false) { ask("grant", "uid" to uid, "input" to input, "on" to on) } }
    val microphone = rememberLauncherForActivityResult(RequestPermission()) { if (it) grant("audio", true) else main.toast(refused) }
    val chooser = rememberChooser(main, uid)
    Column(modifier) {
        val (uses, granted) = form.optJSONObject("uses") to form.optJSONObject("granted")
        if (uses?.optBoolean("audio") == true) Toggle(stringResource(R.string.react_sound), granted?.optBoolean("audio") == true) { on ->
            if (on && main.checkSelfPermission(RECORD_AUDIO) != PERMISSION_GRANTED) microphone.launch(RECORD_AUDIO) else grant("audio", on)
        }
        SoundNetworkGrants(form) { input, on -> grant(input, on) }
        if (uses?.optBoolean("touch") == true) {
            Toggle(stringResource(R.string.react_touch), granted?.optBoolean("touch") == true) { grant("touch", it) }
        }
        if (uses?.optBoolean("taps") == true) {
            Toggle(stringResource(R.string.react_taps), granted?.optBoolean("taps") == true) { grant("taps", it) }
        }
        MediaGrants(form) { input, on -> grant(input, on) }
        if (form.optBoolean("video")) Reeler(main, uid, form, set)
        val fields = form.optJSONArray("fields") ?: JSONArray()
        val renamed = if (form.optBoolean("video")) REEL.keys else if (form.optJSONObject("web")?.optBoolean("own_volume") == false || form.optBoolean("sounds")) setOf("volume") else emptySet()
        val all = (0 until fields.length()).map(fields::getJSONObject).map { if (it.optString("key") in renamed) reeled(it) else it }
        val folded = all.filter { it.getString("kind") == "group" && it.optBoolean("collapsed") != (it.getString("key") in flipped) }.map { it.getString("key") }
        all.filter { it.optBoolean("visible") && it.optString("group") !in folded }.forEach { field ->
            val key = field.getString("key")
            val pick = { choose: Boolean -> if (choose) chooser(field) else main.act { main.unchoose(uid, key) }.let { } }
            Field(field, key in folded, { flipped = if (key in flipped) flipped - key else flipped + key }, set, { main.act { ask("press", "uid" to uid, "key" to key) } }, pick) { text ->
                main.act { ask("secret", "uid" to uid, "key" to key, "text" to text) }
            }
        }
    }
}

private val REEL = mapOf(
    "speed" to (R.string.video_speed to 0), "start" to (R.string.video_start to 0), "end" to (R.string.video_end to R.string.video_end_help),
    "volume" to (R.string.video_volume to 0), "zoom" to (R.string.video_zoom to 0), "x" to (R.string.video_x to 0), "y" to (R.string.video_y to 0),
)

@Composable
private fun reeled(field: JSONObject): JSONObject {
    val (label, help) = REEL[field.optString("key")] ?: return field
    return JSONObject(field.toString()).put("label", stringResource(label)).apply { if (help != 0) put("help", stringResource(help)) }
}

fun options(list: JSONArray?) =
    (0 until (list?.length() ?: 0)).map { list!!.getJSONObject(it) }.map { it.get("value").toString() to it.getString("label") }
