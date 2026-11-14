package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import org.json.JSONObject

suspend fun nowOn(): String? = (ask("current") as? JSONObject)?.takeUnless { it.isNull("on") }?.optString("on")?.takeIf { it.isNotEmpty() }

@Composable
fun standing(watched: Boolean): JSONObject {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val report by produceState(JSONObject(), watched, lifecycle) {
        if (!watched) return@produceState
        lifecycle.repeatOnLifecycle(Lifecycle.State.STARTED) {
            while (true) {
                value = withContext(Dispatchers.IO) { runCatching { JSONObject(Native.diagnostics()) }.getOrDefault(JSONObject()) }
                delay(WATCH_MS)
            }
        }
    }
    return report
}

@Composable
fun Standing(report: JSONObject) {
    val (ph, title, text) = when {
        report.optString("auto") == "still" -> Triple(Ph.Pause, R.string.still_heavy, R.string.still_heavy_why)
        report.optString("reason") in BATTERY -> Triple(Ph.BatteryLow, R.string.lowered_battery, R.string.lowered_battery_why)
        report.optString("reason") == "heat" -> Triple(Ph.ThermometerHot, R.string.lowered_heat, R.string.lowered_heat_why)
        else -> return
    }
    Row(Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e2).testTag("standing"), Arrangement.spacedBy(Measure.e3)) {
        Glyph(ph, null, Modifier.padding(top = 2.dp), tint = tones.mist, size = Measure.iconSquare)
        Column(Modifier.weight(1f)) {
            Text(stringResource(title), style = Type.row)
            Text(stringResource(text), color = tones.mist, style = Type.foot)
        }
    }
}

@Composable
fun InUse(modifier: Modifier = Modifier) = Row(modifier.testTag("in_use"), Arrangement.spacedBy(Measure.e1), androidx.compose.ui.Alignment.CenterVertically) {
    Glyph(Ph.CheckCircle, null, tint = tones.moon, lit = true, size = Measure.iconFold)
    Text(stringResource(R.string.in_use), color = tones.frost, style = Type.foot)
}

private val BATTERY = setOf("battery", "saver")
private const val WATCH_MS = 2000L
