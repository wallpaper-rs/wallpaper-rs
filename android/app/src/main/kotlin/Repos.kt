package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun repoName(bundled: Boolean, name: String) = if (bundled) stringResource(R.string.repo_examples) else name

fun JSONArray?.objects() = this?.let { list -> List(list.length(), list::getJSONObject) }.orEmpty()

@Composable
fun Repos(main: Main, status: JSONObject, run: Run, add: () -> Unit) {
    var changed by remember { mutableIntStateOf(0) }
    var exploring by remember { mutableStateOf(false) }
    val shown by loaded(JSONObject(), changed, status.optInt("offers"), status.optLong("last_sync")) {
        account("repos") as? JSONObject ?: JSONObject()
    }
    fun act(work: suspend () -> Unit) = run("") {
        work()
        changed++
        withContext(Dispatchers.IO) { Updates.schedule(main) }
    }
    Group(stringResource(R.string.repos))
    if (shown.optJSONArray("repos").objects().isEmpty()) Note(stringResource(R.string.repos_none))
    shown.optJSONArray("offers").objects().forEach { offer ->
        val key = offer.optJSONArray("fingerprints")?.optString(0).orEmpty().take(16)
        ListItem(
            headlineContent = { Text(stringResource(R.string.repo_offer, offer.getString("name"), key)) },
            supportingContent = { Text(offer.getString("url").substringBefore('?')) },
            trailingContent = {
                Row {
                    WordAction({ act { account("offer", "repo" to offer.getString("id"), "add" to true) } }) { Text(stringResource(R.string.repo_add)) }
                    WordAction({ act { account("offer", "repo" to offer.getString("id"), "add" to false) } }) { Text(stringResource(R.string.repo_decline)) }
                }
            },
        )
    }
    shown.optJSONArray("repos").objects().forEach { repo ->
        ListItem(
            headlineContent = { Text(repoName(repo.optBoolean("bundled"), repo.getString("name"))) },
            supportingContent = { Text(stringResource(R.string.repo_packages, repo.optInt("packages"))) },
            trailingContent = {
                WordAction({ act { account("remove_repo", "repo_id" to repo.getString("repo_id")) } }) { Text(stringResource(R.string.remove)) }
            },
        )
    }
    Column(Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e3), Arrangement.spacedBy(Measure.e3)) {
        if (shown.optJSONArray("repos").objects().isNotEmpty()) Action({ exploring = true }, Modifier.fillMaxWidth().testTag("explore_repos")) {
            Glyph(Ph.Compass, null, Modifier.padding(end = Measure.e2), size = Measure.iconSmall)
            Text(stringResource(R.string.repo_browse))
        }
        QuietAction(add, Modifier.fillMaxWidth()) { Text(stringResource(R.string.repo_add_title)) }
    }
    if (shown.optJSONArray("repos").objects().isNotEmpty()) RepoUpdates(main, run)
    val examples = shown.optJSONArray("repos").objects().filter { it.optBoolean("bundled") }.map { it.getString("repo_id") }.toSet()
    if (exploring) Explore(main, run, examples) { exploring = false }
}

@Composable
private fun Explore(main: Main, run: Run, examples: Set<String>, close: () -> Unit) {
    var query by rememberSaveable { mutableStateOf("") }
    var installed by remember { mutableIntStateOf(0) }
    val found by loaded(JSONArray(), query, installed) { account("browse", "query" to query) as? JSONArray ?: JSONArray() }
    var voted by remember { mutableStateOf(JSONObject()) }
    val counted by loaded(JSONObject(), found) { votes(found.objects().map { it.getString("repo_id") + "/" + it.getString("id") }) }
    val items = found.objects().sortedByDescending { it.optBoolean("featured") }
    AlertDialog(
        onDismissRequest = close,
        confirmButton = { WordAction(close) { Text(stringResource(R.string.close)) } },
        title = { Text(stringResource(R.string.repo_browse)) },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                OutlinedTextField(query, { query = it }, Modifier.fillMaxWidth(), singleLine = true, label = { Text(stringResource(R.string.repo_search)) })
                Text(stringResource(R.string.votes_note), style = MaterialTheme.typography.bodySmall)
                if (items.isEmpty()) Text(stringResource(R.string.repo_nothing))
                LazyColumn(Modifier.heightIn(max = 360.dp)) {
                    itemsIndexed(items) { index, item ->
                        val featured = item.optBoolean("featured")
                        if (index == 0 && featured || !featured && items.getOrNull(index - 1)?.optBoolean("featured") == true) {
                            Text(stringResource(if (featured) R.string.repo_featured_section else R.string.repo_all), style = MaterialTheme.typography.titleSmall)
                        }
                        val id = item.getString("id")
                        ListItem(
                            leadingContent = { Cover(item.getString("repo_id") + "/" + id + "/" + item.getString("version")) },
                            headlineContent = { Text(item.optString("title").ifEmpty { id }) },
                            supportingContent = {
                                val mature = stringResource(R.string.mature).takeIf { item.optBoolean("mature") }
                                val mark = stringResource(R.string.repo_featured).takeIf { featured }
                                Column {
                                    Text(listOfNotNull(mark, repoName(item.getString("repo_id") in examples, item.getString("repo")), item.getString("version"), size(item.optLong("size")), mature).joinToString(" · "))
                                    val spec = item.getString("repo_id") + "/" + id
                                    val counts = voted.optJSONObject(spec) ?: counted.optJSONObject("items")?.optJSONObject(spec)
                                    if (item.getString("repo_id") == counted.optString("repo_id").takeIf { !counted.isNull("repo_id") }) {
                                        Votes(spec, counts, counted.optBoolean("shown"), run) { voted = JSONObject(voted.toString()).put(spec, it) }
                                    }
                                }
                            },
                            trailingContent = {
                                if (item.optBoolean("installed")) Text(stringResource(R.string.repo_installed))
                                else WordAction({
                                    run("") {
                                        main.complete(account("install", "repo_id" to item.getString("repo_id"), "id" to item.getString("id")))
                                        installed++
                                        main.changed()
                                    }
                                }) { Text(stringResource(R.string.repo_install)) }
                            },
                        )
                    }
                }
            }
        },
    )
}

@Composable
private fun Cover(spec: String) {
    val image by preview(spec)
    val frame = Modifier.size(72.dp, 40.dp).clip(RoundedCornerShape(4.dp))
    image?.let { Image(it, null, frame, contentScale = ContentScale.Crop) }
        ?: Box(frame.background(MaterialTheme.colorScheme.surfaceVariant))
}
