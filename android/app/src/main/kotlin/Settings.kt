package io.github.wallpaper_rs

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.produceState
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import org.json.JSONObject

@Composable
fun Settings(main: Main, account: () -> Unit = {}) {
    var changes by remember { mutableIntStateOf(0) }
    var query by rememberSaveable { mutableStateOf(Jump.find.value.orEmpty()) }
    LaunchedEffect(Unit) { Jump.find.value = null }
    var unfolded by rememberSaveable { mutableIntStateOf(0) }
    val values by loaded(emptyMap<String, String>(), changes) { SETTING_KEYS.associateWith { Native.setting(it, null) } }
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    val report by produceState(JSONObject(), lifecycle) {
        lifecycle.repeatOnLifecycle(Lifecycle.State.STARTED) {
            while (true) {
                value = withContext(Dispatchers.IO) { JSONObject(Native.diagnostics()) }
                delay(1000)
            }
        }
    }
    val change = { work: () -> Any? ->
        main.act {
            withContext(Dispatchers.IO) { work() }
            changes++
        }
        Unit
    }
    val set = { key: String, value: String -> change { stored(main, key, value) } }
    val knobs = Knobs(main, values, report, changes, set, change)
    val labels = SETTING_LINES.map { stringResource(it.label) }
    val accountLabel = stringResource(R.string.account)
    val context = LocalContext.current
    val linked by loaded<String?>(null, main.revision.intValue) { linkedName(Sync.status(), context.online()) }
    val linkedLabel = linked?.let { stringResource(R.string.connected_to, it) }
    val searching = query.isNotBlank()
    val found = SETTING_LINES.filterIndexed { i, _ -> finds(query, labels[i]) }
    val more = stringResource(R.string.more_options)
    LazyColumn(Modifier.testTag("settings")) {
        item { Header(stringResource(R.string.settings)) }
        item { Finder(query, stringResource(R.string.search_settings)) { query = it } }
        if (finds(query, accountLabel)) item { Setting(accountLabel, Modifier.testTag("account"), linkedLabel, onClick = account) }
        Block.entries.forEach { block ->
            val bit = 1 shl block.ordinal
            val open = unfolded and bit != 0
            val mine = SETTING_LINES.filter { it.block == block }
            val shown = if (searching) mine.filter { it in found } else mine.filter { !it.more || open }
            if (shown.isEmpty()) return@forEach
            item(key = block.name) { Group(stringResource(block.label), Modifier.testTag("block_${block.name}")) }
            items(shown, key = { it.label }) { it.draw(knobs) }
            if (!searching && mine.any { it.more }) {
                item(key = "more_${block.name}") { Fold(more, open, Modifier.testTag("more_${block.name}")) { unfolded = unfolded xor bit } }
            }
        }
        if (searching && found.isEmpty() && !finds(query, accountLabel)) {
            item { Blank(stringResource(R.string.no_setting_found), stringResource(R.string.no_setting_found_text, query.trim()), Modifier.testTag("nothing_found")) }
        }
    }
}

private fun stored(main: Main, key: String, value: String) {
    Native.setting(key, value)
    Unveil.load()
    if (key == Updates.SETTING || key == Updates.AUTO) Updates.schedule(main)
    if (key == Updates.AUTO && value == "true") Native.account("{\"op\": \"refresh_repos\", \"force\": false}")
    if (key == Sync.NETWORK) Sync.network(main)
}
