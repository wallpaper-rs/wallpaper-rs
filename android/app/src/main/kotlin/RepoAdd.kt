package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType

@Composable
fun RepoAdd(main: Main, run: Run, close: (() -> Unit)? = null, added: () -> Unit) {
    var url by rememberSaveable { mutableStateOf("") }
    var print by rememberSaveable { mutableStateOf("") }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    val checking = stringResource(R.string.repo_checking)
    val host = hostOf(url)
    val add = {
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
    }
    TaskPage(foot = {
        Action(add, Modifier.weight(1f).testTag("repo_add"), enabled = url.isNotBlank()) { Text(stringResource(R.string.repo_add_url)) }
    }) {
        Header(stringResource(R.string.repo_add_title), close)
        Note(stringResource(R.string.repo_about))
        TextBox(stringResource(R.string.repo_url), url, { url = it; problem = null }, example = stringResource(R.string.repo_url_example))
        TextBox(
            stringResource(R.string.repo_fingerprint), print, { print = it; problem = null },
            problem = problem, example = stringResource(R.string.repo_fingerprint_help), kind = KeyboardType.Ascii, done = add,
        )
        if (host.isNotEmpty()) Note(stringResource(R.string.repo_sent, host))
        Spacer(Modifier.height(Measure.e4))
    }
}
