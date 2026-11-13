package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import org.json.JSONObject

fun sentTo(kind: JSONObject, hosts: List<String>) =
    hosts.joinToString(", ").ifEmpty { null } ?: kind.optString("kind")

@Composable
fun ProviderForm(main: Main, kind: JSONObject, current: JSONObject?, run: Run, close: () -> Unit) {
    val name = kind.getString("kind")
    val fields = kind.getJSONArray("options").objects()
    val values = remember { mutableStateMapOf<String, String>().apply { fields.forEach { put(it.getString("key"), current?.optJSONObject("options")?.optString(it.getString("key")) ?: it.getString("default")) } } }
    val secret = kind.optJSONObject("secret")
    var typed by remember { mutableStateOf("") }
    var on by remember { mutableStateOf(false) }
    var problem by remember { mutableStateOf<String?>(null) }
    val hosts = kind.optJSONArray("hosts").strings().ifEmpty { listOf(hostOf(values["url"] ?: values["server"].orEmpty())).filter(String::isNotEmpty) }
    val downloading = stringResource(R.string.provider_downloading)
    fun save() = run(if (on) downloading else "") {
        try {
            val id = current?.getString("id") ?: account("provider_add", "kind" to name, "options" to JSONObject(values.toMap()), "secret" to typed, "on" to on) as String
            if (current != null) {
                fields.map { it.getString("key") }.filter { values[it] != current.optJSONObject("options")?.optString(it) }.forEach { account("provider_set", "id" to id, "key" to it, "value" to values[it]) }
                if (typed.isNotEmpty()) account("provider_secret", "id" to id, "secret" to typed)
            }
            close()
            if (on) main.report(account("provider_sync", "id" to id, "force" to true))
        } catch (e: Refused) {
            problem = main.told(e)
        }
    }
    AlertDialog(
        onDismissRequest = close,
        title = { Text(stringResource(kindTitle(name))) },
        confirmButton = { TextButton(::save) { Text(stringResource(if (current == null) R.string.provider_add else R.string.provider_save)) } },
        dismissButton = { TextButton(close) { Text(stringResource(R.string.cancel)) } },
        text = {
            Column(Modifier.verticalScroll(rememberScrollState()), Arrangement.spacedBy(8.dp)) {
                Text(stringResource(kindAbout(name)))
                fields.forEach { field ->
                    val key = field.getString("key")
                    val label = stringResource(optionLabel(key))
                    if (field.getString("type") == "pick") {
                        val options = field.optJSONArray("values").strings().map { it to (valueLabel(key, it)?.let { id -> stringResource(id) } ?: it) }
                        Choices(label, options, values[key].orEmpty()) { values[key] = it }
                    } else {
                        val number = if (field.getString("type") == "int") KeyboardType.Number else if (field.getString("type") == "url") KeyboardType.Uri else KeyboardType.Text
                        OutlinedTextField(values[key].orEmpty(), { values[key] = it; problem = null }, Modifier.fillMaxWidth(), singleLine = true, label = { Text(label) }, keyboardOptions = KeyboardOptions(keyboardType = number))
                    }
                }
                secret?.let {
                    val label = stringResource(if (it.getString("name") == "password") R.string.secret_password else R.string.secret_key)
                    val hint = if (it.getBoolean("required")) null else stringResource(R.string.secret_optional)
                    val held = current?.optJSONObject("secret")?.optBoolean("held") == true
                    OutlinedTextField(
                        typed, { typed = it; problem = null }, Modifier.fillMaxWidth(), singleLine = true, label = { Text(listOfNotNull(label, hint).joinToString(" · ")) },
                        visualTransformation = PasswordVisualTransformation(), supportingText = { if (held) Text(stringResource(R.string.secret_kept)) },
                    )
                }
                if (fields.any { it.getString("type") == "url" && values[it.getString("key")].orEmpty().trim().startsWith("http://") }) {
                    Text(stringResource(R.string.provider_plain), color = MaterialTheme.colorScheme.error, style = MaterialTheme.typography.bodySmall)
                }
                Text(stringResource(kindSent(name), sentTo(kind, hosts)), style = MaterialTheme.typography.bodySmall)
                if (current == null) Toggle(stringResource(R.string.provider_turn_on), on) { on = it }
                problem?.let { Text(it, color = MaterialTheme.colorScheme.error) }
            }
        },
    )
}
