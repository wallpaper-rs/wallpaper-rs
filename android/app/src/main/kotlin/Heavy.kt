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
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

fun heavy(report: JSONObject) = report.optString("auto") == "still"

fun playAnyway(main: Main) = main.act { withContext(Dispatchers.IO) { Native.setting(QUALITY, LOWEST) } }

@Composable
fun PlayAnyway(main: Main, modifier: Modifier = Modifier) =
    Action({ playAnyway(main) }, modifier.fillMaxWidth().padding(horizontal = Measure.margin).testTag("play_anyway")) {
        Glyph(Ph.Play, null, size = Measure.iconFold)
        Text(stringResource(R.string.play_anyway), style = Type.row)
    }

@Composable
fun HeavyNotice(main: Main, uid: String, title: String, report: JSONObject) {
    val still = heavy(report)
    val first by loaded(false, uid, still) { still && ask("tell", "uid" to uid) == true }
    var gone by remember(uid) { mutableStateOf(false) }
    if (!first || gone || !still) return
    Tell(stringResource(R.string.heavy_notice, title), Modifier.padding(vertical = Measure.e2).testTag("heavy_notice"), Ph.Info) {
        Row(horizontalArrangement = Arrangement.spacedBy(Measure.e1)) {
            WordAction({ gone = true }) { Text(stringResource(R.string.heavy_not_now)) }
            WordAction({ gone = true; playAnyway(main) }) { Text(stringResource(R.string.play_anyway), color = tones.moon) }
        }
    }
}

private const val QUALITY = "playback.quality"
private const val LOWEST = "min-q1"
