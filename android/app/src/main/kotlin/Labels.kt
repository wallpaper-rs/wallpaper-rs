package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.InputChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

private const val STARS = 5

private val TAG_WORDS = mapOf(
    "example" to R.string.tag_word_example,
    "water" to R.string.tag_word_water,
    "pointer" to R.string.tag_word_pointer,
    "sound" to R.string.tag_word_sound,
    "abstract" to R.string.tag_word_abstract,
    "music" to R.string.tag_word_music,
    "space" to R.string.tag_word_space,
    "script" to R.string.tag_word_script,
    "audio" to R.string.tag_word_audio,
    "media" to R.string.tag_word_media,
    "landscape" to R.string.tag_word_landscape,
    "depth" to R.string.tag_word_depth,
    "winter" to R.string.tag_word_winter,
    "particles" to R.string.tag_word_particles,
    "3d" to R.string.tag_word_three_d,
)

@Composable
fun tagWord(tag: String) = TAG_WORDS[tag.lowercase()]?.let { stringResource(it) } ?: tag

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Labels(main: Main, item: Item) = Column(Modifier.padding(horizontal = 16.dp), Arrangement.spacedBy(4.dp)) {
    var tagging by remember { mutableStateOf(false) }
    var renaming by remember { mutableStateOf(false) }
    val uids = listOf(item.uid)
    Row(verticalAlignment = Alignment.CenterVertically) {
        (1..STARS).forEach { star ->
            val lit = star <= item.rating
            IconButton({ main.act { ask("rate", "uids" to uids, "rating" to if (item.rating == star) 0 else star) } }) {
                val tint = if (lit) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.outlineVariant
                Glyph(Ph.Star, stringResource(R.string.rate_stars, star), lit = lit, tint = tint)
            }
        }
    }
    FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        item.tags.forEach { tag ->
            val word = tagWord(tag)
            InputChip(
                false, { main.act { ask("retag", "uids" to uids, "add" to emptyList<String>(), "remove" to listOf(tag)) } }, { Text(word) },
                trailingIcon = { Glyph(Ph.X, stringResource(R.string.tag_remove, word)) },
            )
        }
    }
    Row {
        WordAction({ tagging = true }) { Text(stringResource(R.string.tag_add)) }
        WordAction({ renaming = true }) { Text(stringResource(R.string.rename)) }
    }
    if (tagging) NameSheet(stringResource(R.string.tag_add), stringResource(R.string.tag_name), "", stringResource(R.string.save), { tagging = false }) { text ->
        main.act { ask("retag", "uids" to uids, "add" to text.split(',').map(String::trim).filter(String::isNotEmpty), "remove" to emptyList<String>()) }
    }
    if (renaming) NameSheet(stringResource(R.string.rename), stringResource(R.string.title_label), item.title, stringResource(R.string.save), { renaming = false }) { text ->
        main.act { ask("rename", "uid" to item.uid, "title" to text) }
    }
}
