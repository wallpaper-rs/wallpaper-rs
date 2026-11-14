package io.github.wallpaper_rs

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.ui.Alignment
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONArray
import org.json.JSONObject

private val LOOKS = mapOf(
    "none" to R.string.look_none, "warm" to R.string.look_warm, "cool" to R.string.look_cool,
    "golden" to R.string.look_golden, "vivid" to R.string.look_vivid, "punch" to R.string.look_punch,
    "muted" to R.string.look_muted, "matte" to R.string.look_matte, "fade" to R.string.look_fade,
    "mono" to R.string.look_mono, "noir" to R.string.look_noir, "sepia" to R.string.look_sepia,
    "dusk" to R.string.look_dusk, "mint" to R.string.look_mint, "rose" to R.string.look_rose, "night" to R.string.look_night,
)
private val SLIDERS = mapOf(
    "strength" to R.string.filter_strength, "exposure" to R.string.filter_exposure, "contrast" to R.string.filter_contrast,
    "saturation" to R.string.filter_saturation, "warmth" to R.string.filter_warmth, "hue" to R.string.filter_hue,
)

@Composable
private fun Looks(label: String, names: List<Pair<String, String>>, swatches: JSONObject?, chosen: String, choose: (String) -> Unit) =
    Column(Modifier.padding(horizontal = 16.dp, vertical = 8.dp)) {
        Text(label, style = MaterialTheme.typography.titleSmall)
        Row(Modifier.horizontalScroll(rememberScrollState()), Arrangement.spacedBy(8.dp)) {
            names.forEach { (value, text) ->
                val colors = swatches?.optJSONArray(value)?.let { all -> (0 until all.length()).map { Color(all.getLong(it).toInt()) } }.orEmpty()
                val border = if (value == chosen) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.outlineVariant
                Column(
                    Modifier.clickable { choose(value) }.border(if (value == chosen) 3.dp else 1.dp, border, RoundedCornerShape(8.dp)).padding(4.dp),
                    horizontalAlignment = Alignment.CenterHorizontally,
                ) {
                    Row(Modifier.size(72.dp, 40.dp).clip(RoundedCornerShape(4.dp))) { colors.forEach { Box(Modifier.weight(1f).fillMaxHeight().background(it)) } }
                    Text(text, style = MaterialTheme.typography.labelSmall)
                }
            }
        }
    }

@Composable
fun Grade(main: Main) = Column {
    var lock by remember { mutableStateOf(false) }
    var changes by remember { mutableIntStateOf(0) }
    val shown by loaded(JSONObject(), lock, changes) { ask("filter", "lock" to lock) as JSONObject }
    val filter = shown.optJSONObject("filter") ?: JSONObject()
    val send = { chosen: JSONObject? ->
        main.act {
            ask("set_filter", "lock" to lock, "filter" to chosen)
            changes++
        }
    }
    val change = { key: String, value: Any -> send(JSONObject(filter.toString()).put(key, value)) }
    Text(stringResource(R.string.filter), Modifier.padding(start = Measure.margin, end = Measure.margin, top = Measure.e3), style = Type.row)
    val screens = listOf("home" to stringResource(R.string.home_screen), "lock" to stringResource(R.string.lock_screen))
    Choices(null, screens, if (lock) "lock" else "home") { lock = it == "lock" }
    val own = shown.optBoolean("own")
    if (lock) Toggle(stringResource(R.string.filter_follows_home), !own) { follows ->
        send(if (follows) null else filter)
    }
    val looks = shown.optJSONArray("looks") ?: JSONArray()
    val names = (0 until looks.length()).map(looks::getString).map { it to stringResource(LOOKS[it] ?: R.string.look_none) }
    Looks(stringResource(R.string.filter_look), names, shown.optJSONObject("swatches"), filter.optString("look", "none")) { change("look", it) }
    val fields = shown.optJSONArray("fields") ?: JSONArray()
    (0 until fields.length()).map(fields::getJSONObject).forEach { field ->
        val key = field.getString("key")
        val labelled = JSONObject(field.toString()).put("label", stringResource(SLIDERS[key] ?: R.string.filter))
        Slide(labelled, Modifier.fillMaxWidth(), true) { change(key, it) }
    }
    WordAction({ send(JSONObject()) }, Modifier.padding(horizontal = 8.dp)) { Text(stringResource(R.string.show_as_it_is)) }
    Note(stringResource(R.string.filter_note))
}
