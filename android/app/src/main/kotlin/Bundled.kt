package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import org.json.JSONArray
import org.json.JSONObject

private val SAMPLE = 112.dp

suspend fun bundled(): List<JSONObject> {
    val repos = (account("repos") as? JSONObject)?.optJSONArray("repos").objects()
        .filter { it.optBoolean("bundled") }.map { it.getString("repo_id") }.toSet()
    if (repos.isEmpty()) return emptyList()
    return (account("browse", "query" to "") as? JSONArray).objects().filter { it.getString("repo_id") in repos }
}

@Composable
fun Bundled(main: Main) {
    val found by loaded(emptyList<JSONObject>(), main.revision.intValue) { bundled() }
    if (found.isEmpty()) return
    Row(Modifier.fillMaxWidth().padding(start = Measure.margin, end = Measure.e2, top = Measure.e8), verticalAlignment = Alignment.CenterVertically) {
        Text(stringResource(R.string.example_scenes), Modifier.weight(1f), style = Type.titleS)
        WordAction(main::examples, Modifier.testTag("add_examples")) { Text(stringResource(R.string.add_them)) }
    }
    LazyRow(
        Modifier.testTag("bundled"),
        contentPadding = PaddingValues(horizontal = Measure.margin, vertical = Measure.e2),
        horizontalArrangement = Arrangement.spacedBy(Measure.e3),
    ) {
        items(found, key = { it.getString("repo_id") + "/" + it.getString("id") }) { item ->
            Column(Modifier.width(SAMPLE)) {
                val picture by preview(item.getString("repo_id") + "/" + item.getString("id") + "/" + item.getString("version"))
                val frame = Modifier.fillMaxWidth().aspectRatio(16f / 10f).clip(RoundedCornerShape(Measure.rControl))
                picture?.let { Image(it, null, frame, contentScale = ContentScale.Crop) } ?: Column(frame.background(tones.nightHigh)) {}
                Text(item.optString("title").ifEmpty { item.getString("id") }, Modifier.padding(top = Measure.e1), style = Type.foot, maxLines = 1, overflow = TextOverflow.Ellipsis)
            }
        }
    }
}
