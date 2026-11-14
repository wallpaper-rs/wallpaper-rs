package io.github.wallpaper_rs

import android.content.Context
import android.net.ConnectivityManager
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONArray
import org.json.JSONObject

fun updatesWanted(refresh: String?, automatic: String?) = refresh != "false" || automatic == "true"

fun askNote(asks: List<String>) = asks.joinToString(", ").takeIf { it.isNotEmpty() }

fun asked(update: JSONObject) = update.optJSONArray("asks")?.let { askNote(List(it.length(), it::getString)) }

fun online(context: Context) = context.getSystemService(ConnectivityManager::class.java)?.let { it.getNetworkCapabilities(it.activeNetwork) } != null

@Composable
fun RepoUpdates(main: Main, run: Run) {
    var checked by remember { mutableIntStateOf(0) }
    var settled by remember { mutableStateOf(false) }
    val updates by loaded(JSONArray(), checked, main.revision.intValue) { account("updates") as? JSONArray ?: JSONArray() }
    LaunchedEffect(updates) { Updates.found(updates.length()) }
    val listed = List(updates.length(), updates::getJSONObject)
    val context = LocalContext.current
    val offline = remember(checked) { !online(context) }
    val update: suspend (String) -> Unit = { uid -> main.complete(account("update", "uid" to uid)) }
    if (listed.isNotEmpty()) Group(stringResource(R.string.repo_updates))
    listed.forEach { found ->
        val from = stringResource(R.string.repo_update_from, found.getString("installed"), found.getString("available"))
        val asks = asked(found)?.let { stringResource(R.string.repo_update_asks, it) }
        Setting(found.getString("title").ifEmpty { found.getString("id") }, Modifier.testTag("update"), sub = listOfNotNull(from, asks).joinToString("\n"), trailing = {
            WordAction({
                run("") {
                    update(found.getString("uid"))
                    checked++
                    main.changed()
                }
            }) { Text(stringResource(R.string.repo_update)) }
        })
    }
    if (listed.isNotEmpty()) {
        val words = context.resources
        Action({
            run("") {
                listed.forEach { update(it.getString("uid")) }
                settled = true
                checked++
                main.changed()
                main.toast(words.getQuantityString(R.plurals.updated_count, listed.size, listed.size))
            }
        }, Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e3).testTag("update_all")) {
            Glyph(Ph.ArrowClockwise, null)
            Text(stringResource(R.string.update_all))
        }
    } else if (settled) {
        StateLine(stringResource(R.string.all_updated), Tone.Sprout, Modifier.padding(horizontal = Measure.margin).testTag("all_updated"))
    }
    if (offline) Note(stringResource(R.string.updates_offline))
    WordAction({
        run("") {
            account("refresh_repos", "force" to true)
            settled = true
            checked++
        }
    }, Modifier.padding(horizontal = Measure.e2)) {
        Text(stringResource(R.string.repo_check))
    }
}
