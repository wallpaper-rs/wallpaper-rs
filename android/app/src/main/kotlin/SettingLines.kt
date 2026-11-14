package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

enum class Block(val label: Int) { Wallpapers(R.string.block_wallpapers), Screen(R.string.block_screen), Phone(R.string.block_phone) }

class Knobs(
    val main: Main,
    val values: Map<String, String>,
    val report: JSONObject,
    val changes: Int,
    val set: (String, String) -> Unit,
    val change: (() -> Any?) -> Unit,
) {
    fun on(key: String, value: String) = values[key] == value

    operator fun get(key: String) = values[key].orEmpty()
}

class Line(val label: Int, val block: Block, val more: Boolean = false, val always: Boolean = true, val draw: @Composable (Knobs) -> Unit)

@Composable
private fun flip(k: Knobs, label: Int, key: String, on: String, off: String, sub: String? = null) =
    Switch(stringResource(label), k.on(key, on), sub) { k.set(key, if (it) on else off) }

private val WALLPAPERS = listOf(
    Line(R.string.fit, Block.Wallpapers) { Pick(stringResource(R.string.fit), fitWords(), it["fit"]) { v -> it.set("fit", v) } },
    Line(R.string.fade, Block.Wallpapers) { k ->
        Switch(stringResource(R.string.fade), k["transition.kind"] != Passage.NONE, stringResource(passageNote(k.values))) {
            k.set("transition.kind", if (it) Passage.OWN else Passage.NONE)
        }
    },
    Line(R.string.hide_mature, Block.Wallpapers) { flip(it, R.string.hide_mature, "library.hide_mature", "true", "false") },
    Line(R.string.allow_flashing, Block.Wallpapers) {
        flip(it, R.string.allow_flashing, "playback.allow_flashing", "true", "false", stringResource(R.string.allow_flashing_sub))
    },
    Line(R.string.auto_update, Block.Wallpapers) { k ->
        val on = k.on(Updates.AUTO, "true")
        flip(k, R.string.auto_update, Updates.AUTO, "true", "false", stringResource(if (on) R.string.auto_update_on else R.string.auto_update_off))
    },
    Line(R.string.auto_refresh, Block.Wallpapers, more = true) {
        flip(it, R.string.auto_refresh, Updates.SETTING, "true", "false", stringResource(R.string.auto_refresh_sub))
    },
    Line(R.string.show_mature, Block.Wallpapers, more = true) { flip(it, R.string.show_mature, Unveil.KEY, "true", "false") },
    Line(R.string.vote_counts, Block.Wallpapers, more = true) { flip(it, R.string.vote_counts, "repos.vote_counts", "true", "false") },
)

private val SCREEN = listOf(
    Line(R.string.quality, Block.Screen) { k ->
        Pick(stringResource(R.string.quality), qualityWords(), k["playback.quality"], inSub = true, more = sevenLevels()) { k.set("playback.quality", it) }
        CostWarning(k.report)
    },
    Line(R.string.battery_fps, Block.Screen) { k ->
        Pick(stringResource(R.string.battery_fps), animationWords(), k["playback.max_fps"]) { k.set("playback.max_fps", it) }
    },
    Line(R.string.pause_below, Block.Screen) { k -> PausePick(stringResource(R.string.pause_below), k["pause.on_battery"]) { k.set("pause.on_battery", it) } },
    Line(R.string.video_sound, Block.Screen) { k ->
        flip(k, R.string.video_sound, "playback.muted", "false", "true")
        if (k.on("playback.muted", "false")) Loudness(k.values[VOLUME]) { k.set(VOLUME, it) }
    },
    Line(R.string.duck, Block.Screen) { flip(it, R.string.duck, "playback.duck", "true", "false") },
    Line(R.string.scene_capture, Block.Screen) { flip(it, R.string.scene_capture, "playback.scene_capture", "true", "false") },
    Line(R.string.convert_videos, Block.Screen) { k ->
        Pick(stringResource(R.string.convert_videos), convertWords(), k[Convert.SETTING]) { k.set(Convert.SETTING, it) }
    },
    Line(R.string.filter, Block.Screen) { Grade(it.main) },
    Line(R.string.lowest_below, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.lowest_below), lowestWords(), k["playback.lowest_below_percent"]) { k.set("playback.lowest_below_percent", it) }
    },
    Line(R.string.system_colors, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.system_colors), colorWords(), k["system_colors"]) { k.set("system_colors", it) }
    },
    Line(R.string.charger_fps, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.charger_fps), chargerWords(), k["charger_fps"]) { k.set("charger_fps", it) }
    },
    Line(R.string.auto_pace, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.auto_pace), paceWords(), k["playback.auto_pace"]) { k.set("playback.auto_pace", it) }
        PaceNote()
    },
    Line(R.string.hdr_output, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.hdr_output), hdrWords(), k["playback.hdr"]) { k.set("playback.hdr", it) }
    },
    Line(R.string.backend, Block.Screen, more = true) { k -> Pick(stringResource(R.string.backend), backendWords(), k["backend"]) { k.set("backend", it) } },
    Line(R.string.parallax_fill, Block.Screen, more = true) { k ->
        Pick(stringResource(R.string.parallax_fill), fillWords(), k["parallax.fill"]) { k.set("parallax.fill", it) }
        Note(stringResource(R.string.fill_note))
    },
)

val SETTING_LINES = WALLPAPERS + SCREEN + PHONE
