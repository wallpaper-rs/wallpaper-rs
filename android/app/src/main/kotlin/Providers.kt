package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Checkbox
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import java.text.DateFormat
import java.util.Date
import org.json.JSONArray
import org.json.JSONObject

fun Main.report(reply: Any?) {
    val got = (reply as? JSONArray)?.let { list -> (0 until list.length()).sumOf { list.getJSONObject(it).optInt("brought") } } ?: 0
    toast(if (got == 0) getString(R.string.provider_nothing) else resources.getQuantityString(R.plurals.provider_got, got, got))
    changed()
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Providers(main: Main, shown: JSONObject, run: Run, library: (String) -> Unit) {
    var adding by remember { mutableStateOf<JSONObject?>(null) }
    val kinds = shown.optJSONArray("kinds").objects()
    Text(stringResource(R.string.providers), Modifier.padding(horizontal = 16.dp), style = MaterialTheme.typography.titleSmall)
    Note(stringResource(R.string.providers_intro))
    Note(stringResource(if (shown.optString("network") == "any") R.string.provider_network_any else R.string.provider_network_wifi))
    shown.optJSONArray("providers").objects().forEach { provider ->
        Provider(main, provider, kinds.first { it.getString("kind") == provider.getString("kind") }, run, library)
    }
    Text(stringResource(R.string.provider_add_title), Modifier.padding(16.dp, 8.dp), style = MaterialTheme.typography.titleSmall)
    FlowRow(Modifier.padding(horizontal = 16.dp), Arrangement.spacedBy(8.dp)) {
        kinds.forEach { kind -> OutlinedButton({ adding = kind }) { Text(stringResource(kindTitle(kind.getString("kind")))) } }
    }
    adding?.let { ProviderForm(main, it, null, run) { adding = null } }
}

@Composable
private fun Provider(main: Main, provider: JSONObject, kind: JSONObject, run: Run, library: (String) -> Unit) {
    var editing by remember { mutableStateOf(false) }
    var removing by remember { mutableStateOf(false) }
    var crediting by remember { mutableStateOf(false) }
    val (id, name) = provider.getString("id") to provider.getString("kind")
    val on = provider.optBoolean("on")
    val downloading = stringResource(R.string.provider_downloading)
    val count = provider.optInt("count")
    Column(Modifier.padding(16.dp, 8.dp), Arrangement.spacedBy(4.dp)) {
        Text(stringResource(kindTitle(name)) + if (id != name) " ($id)" else "", style = MaterialTheme.typography.titleMedium)
        val error = provider.optJSONObject("last_error")?.let(::refusal)?.let(main::told)
        val last = provider.optLong("last_sync")
        val time = DateFormat.getDateTimeInstance(DateFormat.SHORT, DateFormat.SHORT).format(Date(last * 1000))
        Text(if (last > 0) stringResource(R.string.provider_last, time, count) else stringResource(R.string.provider_never))
        error?.let { Text(it, color = MaterialTheme.colorScheme.error) }
        Toggle(stringResource(R.string.provider_turn_on), on) { now ->
            run(if (now) downloading else "") {
                account("provider_on", "id" to id, "on" to now)
                if (now) main.report(account("provider_sync", "id" to id, "force" to true))
            }
        }
        Note(stringResource(kindSent(name), sentTo(kind, provider.optJSONArray("hosts").strings())), Color.Unspecified)
        Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
            if (on) TextButton({ run(downloading) { main.report(account("provider_sync", "id" to id, "force" to true)) } }) { Text(stringResource(R.string.provider_sync)) }
            if (count > 0) TextButton({ library("tag:from-$id") }) { Text(stringResource(R.string.provider_see, count)) }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
            TextButton({ editing = true }) { Text(stringResource(R.string.provider_options)) }
            if (provider.optJSONArray("credits").objects().isNotEmpty()) TextButton({ crediting = true }) { Text(stringResource(R.string.provider_credits)) }
            TextButton({ removing = true }) { Text(stringResource(R.string.remove)) }
        }
    }
    if (editing) ProviderForm(main, kind, provider, run) { editing = false }
    if (crediting) Credits(provider.optJSONArray("credits").objects()) { crediting = false }
    if (removing) Removing(id, run) { removing = false }
}

@Composable
private fun Credits(credits: List<JSONObject>, close: () -> Unit) = AlertDialog(
    onDismissRequest = close,
    confirmButton = { TextButton(close) { Text(stringResource(R.string.close)) } },
    title = { Text(stringResource(R.string.provider_credits)) },
    text = {
        Column(Modifier.verticalScroll(rememberScrollState()), Arrangement.spacedBy(8.dp)) {
            credits.forEach { Text(listOfNotNull(it.optString("title"), it.optString("attribution").ifEmpty { null }, it.optString("license").ifEmpty { null }).joinToString(" · ")) }
        }
    },
)

@Composable
private fun Removing(id: String, run: Run, close: () -> Unit) {
    var purge by remember { mutableStateOf(false) }
    AlertDialog(
        onDismissRequest = close,
        confirmButton = { TextButton({ close(); run("") { account("provider_remove", "id" to id, "purge" to purge) } }) { Text(stringResource(R.string.remove)) } },
        dismissButton = { TextButton(close) { Text(stringResource(R.string.cancel)) } },
        text = {
            Column {
                Text(stringResource(R.string.provider_remove_question))
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Checkbox(purge, { purge = it })
                    Text(stringResource(R.string.provider_remove_pictures))
                }
            }
        },
    )
}
