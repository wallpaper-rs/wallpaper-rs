package io.github.wallpaper_rs

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import org.json.JSONArray
import org.json.JSONObject

fun updatesWanted(refresh: String?, automatic: String?) = refresh != "false" || automatic == "true"

fun askNote(asks: List<String>) = asks.joinToString(", ").takeIf { it.isNotEmpty() }

fun asked(update: JSONObject) = update.optJSONArray("asks")?.let { askNote(List(it.length(), it::getString)) }

@Composable
fun RepoUpdates(main: Main, run: Run) {
    var checked by remember { mutableIntStateOf(0) }
    val updates by loaded(JSONArray(), checked, main.revision.intValue) { account("updates") as? JSONArray ?: JSONArray() }
    LaunchedEffect(updates) { Updates.found(updates.length()) }
    val listed = List(updates.length(), updates::getJSONObject)
    if (listed.isNotEmpty()) Text(stringResource(R.string.repo_updates), Modifier.padding(horizontal = 16.dp), style = MaterialTheme.typography.titleSmall)
    listed.forEach { update ->
        ListItem(
            headlineContent = { Text(update.getString("title").ifEmpty { update.getString("id") }) },
            supportingContent = {
                Text(stringResource(R.string.repo_update_from, update.getString("installed"), update.getString("available")))
                asked(update)?.let { Text(stringResource(R.string.repo_update_asks, it), style = MaterialTheme.typography.bodySmall) }
            },
            trailingContent = {
                WordAction({
                    run("") {
                        main.complete(account("update", "uid" to update.getString("uid")))
                        checked++
                        main.changed()
                    }
                }) { Text(stringResource(R.string.repo_update)) }
            },
        )
    }
    WordAction({ run("") { account("refresh_repos", "force" to true).also { checked++ } } }, Modifier.padding(horizontal = 8.dp)) {
        Text(stringResource(R.string.repo_check))
    }
}
