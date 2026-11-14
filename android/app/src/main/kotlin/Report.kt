package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

@Composable
fun reportLines(report: JSONObject): List<String> {
    val answers = stringResource(R.string.no) to stringResource(R.string.yes)
    val yes = { flag: String -> if (report.optBoolean(flag)) answers.second else answers.first }
    val tiered = report.optString("tier").isNotEmpty()
    return listOfNotNull(
        stringResource(R.string.gpu, report.optString("gpu").ifEmpty { stringResource(R.string.not_started) }),
        stringResource(R.string.frames, report.optLong("frames"), report.optInt("fps")),
        stringResource(R.string.cap, report.optInt("cap")),
        stringResource(R.string.power, yes("charging"), yes("saver"), report.optInt("thermal")),
        stringResource(levelWords(report.optInt("pace")), report.optString("tier"), report.optString("quality"), report.optInt("pace_fps"))
            .takeIf { tiered },
        stringResource(if (report.optBoolean("measured")) R.string.tier_measured else R.string.tier_detected).takeIf { tiered },
        stringResource(R.string.tier_timed).takeIf { report.optBoolean("timed") && tiered },
        levelReason(report.optString("reason"))?.let { stringResource(R.string.level_reason, stringResource(it)) },
        stringResource(R.string.level_chosen).takeIf { report.optString("reason") == "user" },
        frameNote(report),
        costNote(report),
        hdrState(report.optJSONObject("hdr")),
        stringResource(R.string.particles_cpu).takeIf { report.optString("particles") == "cpu" },
        stringResource(R.string.scripts_stopped).takeIf { report.optString("scripts") == "off" },
        stringResource(R.string.script_stopped).takeIf { report.optString("scripts") == "stopped" },
        stringResource(if (report.optBoolean("stamps")) R.string.timed_gpu else R.string.timed_wait).takeIf { report.has("stamps") },
    ) + report.optJSONArray("left_out").let { left ->
        (0 until (left?.length() ?: 0)).map {
            val effect = left!!.getJSONObject(it)
            stringResource(R.string.left_out, effect.optString("id"), leftOutReason(effect.optString("reason")))
        }
    }
}
