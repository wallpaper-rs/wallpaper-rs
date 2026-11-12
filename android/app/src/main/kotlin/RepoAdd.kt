package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

@Composable
fun RepoAdd(main: Main, run: Run, added: () -> Unit) = Column(Modifier.padding(16.dp), Arrangement.spacedBy(8.dp)) {
    var url by rememberSaveable { mutableStateOf("") }
    var print by rememberSaveable { mutableStateOf("") }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    val checking = stringResource(R.string.repo_checking)
    val host = hostOf(url)
    Text(stringResource(R.string.repo_add_title), style = MaterialTheme.typography.titleSmall)
    Text(stringResource(R.string.repo_about), style = MaterialTheme.typography.bodySmall)
    OutlinedTextField(
        url, { url = it; problem = null }, Modifier.fillMaxWidth(), singleLine = true,
        label = { Text(stringResource(R.string.repo_url)) }, placeholder = { Text(stringResource(R.string.repo_url_example)) },
    )
    OutlinedTextField(
        print, { print = it; problem = null }, Modifier.fillMaxWidth(), minLines = 2,
        label = { Text(stringResource(R.string.repo_fingerprint)) }, isError = problem != null,
        supportingText = { Text(problem ?: stringResource(R.string.repo_fingerprint_help)) },
    )
    if (host.isNotEmpty()) Text(stringResource(R.string.repo_sent, host), style = MaterialTheme.typography.bodySmall)
    Button({
        problem = null
        run(checking) {
            try {
                account("add_anchored", "url" to url.trim(), "fingerprint" to print.trim().ifEmpty { null })
                url = ""
                print = ""
                added()
            } catch (e: Refused) {
                problem = main.told(e).ifBlank { main.getString(R.string.failed_without_reason) }
            }
        }
    }, enabled = url.isNotBlank()) { Text(stringResource(R.string.repo_add_url)) }
}
