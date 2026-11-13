package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

@Composable
fun Sources(main: Main, close: (() -> Unit)? = null, library: (String) -> Unit) {
    var changes by remember { mutableIntStateOf(0) }
    var task by remember { mutableStateOf<String?>(null) }
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
    LazyColumn {
        item(key = "header") { Header(stringResource(R.string.sources), close) }
        item(key = "intro") { Note(stringResource(R.string.sources_intro)) }
        task?.let { waiting -> item(key = "waiting") { Waiting(main, waiting) } }
        item(key = "providers") { Column { Providers(main, providers, run, library) } }
        item(key = "repos") { Column { Repos(main, status, run) } }
    }
}
