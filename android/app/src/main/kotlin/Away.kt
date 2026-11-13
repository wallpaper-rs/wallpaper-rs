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
import org.json.JSONObject

fun awayName(title: String, key: String) = title.ifEmpty { key.take(8) }

data class AwayItem(val title: String, val repo: String = "") {
    companion object {
        fun of(json: JSONObject) = AwayItem(
            awayName(json.optString("title"), json.optString("key")),
            json.optString("repo"),
        )
    }
}

fun awayNote(repo: String) = if (repo.isEmpty()) R.string.away_note else R.string.away_note_repo

@Composable
fun AwayLine(item: AwayItem, modifier: Modifier = Modifier) =
    Text(
        stringResource(awayNote(item.repo), item.title, item.repo),
        modifier,
        style = MaterialTheme.typography.bodySmall,
    )

@Composable
fun AwayNotes(main: Main) {
    val away by loaded(emptyList<AwayItem>(), main.revision.intValue) {
        (account("away") as? JSONArray)?.let { list ->
            List(list.length()) { AwayItem.of(list.getJSONObject(it)) }
        }.orEmpty()
    }
    away.forEach { AwayLine(it, Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) }
}
