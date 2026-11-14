package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

private val LACKS = mapOf(
    "gpu" to R.string.lacks_gpu,
    "display2" to R.string.lacks_display2,
    "spanned_scenes" to R.string.lacks_spanned_scenes,
    "script_geometry" to R.string.lacks_script_geometry,
    "script_storage" to R.string.lacks_script_storage,
    "script_animations" to R.string.lacks_script_animations,
    "script_camera" to R.string.lacks_script_camera,
    "script_hierarchy" to R.string.lacks_script_hierarchy,
    "rgb" to R.string.lacks_rgb,
    "media_info" to R.string.lacks_media_info,
)

@Composable
fun Own(main: Main, uid: String, item: Item?, picture: Boolean, state: JSONObject, form: JSONObject, undecided: List<String>) = Column {
    if (picture && state.optBoolean("changed")) Note(stringResource(R.string.picture_changed), MaterialTheme.colorScheme.error)
    if (picture) Relief(main, uid, state)
    item?.takeIf { it.video && !it.reel }?.let { ConvertVideo(main, it) }
    if (item?.scene == true || item?.reel == true) SceneControls(main, uid, form, undecided)
}

@Composable
fun About(main: Main, uid: String, item: Item, form: JSONObject, open: (String) -> Unit) = Column(Modifier.testTag("more_page")) {
    item.about.takeIf { it.isNotEmpty() }?.let { Text(it, Modifier.padding(horizontal = Measure.margin), tones.mist, style = Type.text) }
    Labels(main, item)
    item.lacks.mapNotNull(LACKS::get).map { stringResource(it) }.takeIf { it.isNotEmpty() }?.let { Note(it.joinToString(" ")) }
    Kin(main, uid, open)
    item.takeIf { it.video || it.scene || it.web }?.let { OwnSound(main, it) }
    if (!item.scene) Kept(main, uid)
    item.takeIf { it.scene && !it.web && !it.flashes }?.let { shown ->
        Toggle(stringResource(R.string.keep_moving), shown.moving) {
            main.act { ask(if (it) "confirm" else "unconfirm", "uid" to uid, "what" to "motion") }
        }
    }
    if (item.scene || item.reel) SceneMore(main, uid, form)
    Exposes(item.exposes)
}

@Composable
fun facts(item: Item): String = listOfNotNull(
    stringResource(if (item.web) R.string.kind_web else if (item.video) R.string.kind_video else if (item.scene) R.string.kind_scene else R.string.kind_photo),
    stringResource(R.string.size, item.width, item.height).takeIf { item.width > 0 },
    stringResource(R.string.duration, item.millis / 60_000, item.millis / 1000 % 60).takeIf { item.millis > 0 },
).joinToString(" · ")
