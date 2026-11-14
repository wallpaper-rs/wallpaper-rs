package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.lazy.LazyColumn
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
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

@Composable
fun Sources(main: Main, close: (() -> Unit)? = null, connect: (() -> Unit)? = null, library: (String) -> Unit) {
    var changes by remember { mutableIntStateOf(0) }
    var task by remember { mutableStateOf<String?>(null) }
    var adding by rememberSaveable { mutableStateOf(false) }
    val status by loaded(JSONObject(), changes, main.revision.intValue) { Sync.status() }
    val providers by loaded(JSONObject(), changes, main.revision.intValue) { account("providers") as? JSONObject ?: JSONObject() }
    val run: Run = { waiting, work ->
        main.act(false) {
            task = waiting
            try {
                work()
            } finally {
                task = null
                changes++
                withContext(Dispatchers.IO) {
                    Updates.schedule(main)
                    ProviderWork.schedule(main)
                }
            }
        }
    }
    if (adding) {
        BackHandler { adding = false }
        return RepoAdd(main, run, { adding = false }) { adding = false }
    }
    LazyColumn(Modifier.testTag("sources")) {
        item(key = "header") { Header(stringResource(R.string.sources), close) }
        item(key = "intro") { Note(stringResource(R.string.sources_intro)) }
        task?.let { waiting -> item(key = "waiting") { Waiting(main, waiting) } }
        item(key = "repos") { Column { Repos(main, status, run) { adding = true } } }
        if (connect != null && linkedName(status, true) == null) item(key = "server") {
            Column {
                Group(stringResource(R.string.your_server))
                Setting(stringResource(R.string.connect_block), Modifier.testTag("connect_server"), stringResource(R.string.connect_server_note), onClick = connect) {
                    Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
                }
            }
        }
        item(key = "providers") { Column { Providers(main, providers, run, library) } }
        item(key = "end") { Spacer(Modifier.height(Measure.e8)) }
    }
}
