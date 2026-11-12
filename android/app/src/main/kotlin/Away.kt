package io.github.wallpaper_rs

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONArray

fun awayName(title: String, key: String) = title.ifEmpty { key.take(8) }

@Composable
fun AwayLine(title: String, modifier: Modifier = Modifier) =
    Text(stringResource(R.string.away_note, title), modifier, style = MaterialTheme.typography.bodySmall)

@Composable
fun AwayNotes(main: Main) {
    val away by loaded(emptyList<String>(), main.revision.intValue) {
        (account("away") as? JSONArray)?.let { list ->
            List(list.length()) { list.getJSONObject(it).let { a -> awayName(a.optString("title"), a.optString("key")) } }
        }.orEmpty()
    }
    away.forEach { AwayLine(it, Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) }
}
