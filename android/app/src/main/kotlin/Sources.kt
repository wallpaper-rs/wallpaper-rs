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
fun Sources(main: Main, library: (String) -> Unit) {
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
        item { Header(stringResource(R.string.sources)) }
        item { Note(stringResource(R.string.sources_intro)) }
        task?.let { waiting -> item { Waiting(main, waiting) } }
        item { Column { Providers(main, providers, run, library) } }
        item { Column { Repos(main, status, run) } }
    }
}
