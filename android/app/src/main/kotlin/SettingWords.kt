package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource

val FIT =
    listOf("fill" to R.string.fill, "fit" to R.string.whole, "stretch" to R.string.stretch, "center" to R.string.center)
private val COLORS = listOf("chosen" to R.string.when_chosen, "always" to R.string.always, "never" to R.string.never)
private val BACKENDS = listOf("auto" to R.string.automatic, "vulkan" to R.string.vulkan, "gl" to R.string.gl)
private val PACES = listOf("detail" to R.string.pace_detail, "smooth" to R.string.pace_smooth)
private val HDRS = listOf("auto" to R.string.hdr_auto, "on" to R.string.hdr_on, "off" to R.string.hdr_off)
private val FILLS = listOf("simple" to R.string.fill_simple, "lama" to R.string.fill_lama, "off" to R.string.fill_off)
private val CONVERTS = listOf("auto" to R.string.convert_auto, "keep" to R.string.convert_keep, "off" to R.string.never)
private val ANIMATIONS = listOf("60" to R.string.motion_fluid, "30" to R.string.motion_medium, "15" to R.string.motion_saving, "10" to R.string.motion_least)
private val SEVEN = listOf(
    Triple("auto", R.string.quality_automatic, R.string.quality_automatic_sub),
    Triple("min-q2", R.string.level_min_q2, 0),
    Triple("min-q1", R.string.level_min_q1, 0),
    Triple("q3", R.string.level_q3, 0),
    Triple("q2", R.string.level_q2, 0),
    Triple("q1", R.string.level_q1, 0),
    Triple("q0", R.string.level_q0, R.string.level_q0_sub),
)
private val QUALITIES = listOf(
    Triple("auto", R.string.quality_automatic, R.string.quality_automatic_sub),
    Triple("q0", R.string.quality_saving, R.string.quality_saving_sub),
    Triple("q3", R.string.quality_top, R.string.quality_top_sub),
)

val SETTING_KEYS = listOf(
    "fit", "playback.max_fps", "charger_fps", "pause.on_battery", "playback.lowest_below_percent", "system_colors", "backend",
    "playback.quality", "playback.auto_pace", "playback.hdr", "parallax.fill", Convert.SETTING, "transition.kind", "playback.muted",
    "playback.duck", "models.allow_download", "playback.allow_flashing", "playback.scene_capture", "library.hide_mature", Unveil.KEY,
    "repos.vote_counts", Updates.SETTING, Updates.AUTO, Sync.NETWORK, VOLUME,
)

@Composable
private fun words(pairs: List<Pair<String, Int>>) = pairs.map { (value, text) -> Word(value, stringResource(text)) }

@Composable
private fun subbed(triples: List<Triple<String, Int, Int>>) =
    triples.map { (value, text, sub) -> Word(value, stringResource(text), sub.takeIf { it != 0 }?.let { stringResource(it) }) }

@Composable
fun fitWords() = words(FIT)

@Composable
fun colorWords() = words(COLORS)

@Composable
fun backendWords() = words(BACKENDS)

@Composable
fun paceWords() = words(PACES)

@Composable
fun hdrWords() = words(HDRS)

@Composable
fun fillWords() = words(FILLS)

@Composable
fun convertWords() = words(CONVERTS)

@Composable
fun qualityWords() = subbed(QUALITIES)

@Composable
fun sevenLevels() = subbed(SEVEN)

@Composable
fun animationWords() = ANIMATIONS.map { (value, text) -> Word(value, stringResource(text), stringResource(R.string.fps, value.toInt())) }

@Composable
fun chargerWords() = listOf("60", "30").map { Word(it, stringResource(R.string.fps, it.toInt())) }

@Composable
fun lowestWords() = listOf(Word("0", stringResource(R.string.never))) +
    listOf("20", "30", "50").map { Word(it, stringResource(R.string.below_short, it.toInt())) }
