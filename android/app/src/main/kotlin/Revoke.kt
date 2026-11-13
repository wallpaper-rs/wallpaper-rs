package io.github.wallpaper_rs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp

@Composable
fun Revoke(main: Main, run: Run, id: String, name: String, close: () -> Unit) {
    var password by remember { mutableStateOf("") }
    val revoked = stringResource(R.string.revoked, name)
    AlertDialog(
        onDismissRequest = close,
        title = { Text(stringResource(R.string.revoke_device, name)) },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                Text(stringResource(R.string.revoke_warning))
                OutlinedTextField(
                    password, { password = it }, singleLine = true, visualTransformation = PasswordVisualTransformation(),
                    label = { Text(stringResource(R.string.account_password)) },
                )
            }
        },
        confirmButton = {
            WordAction({
                close()
                run("") {
                    account("revoke", "device" to id, "password" to password)
                    main.toast(revoked)
                }
            }, enabled = password.isNotEmpty()) { Text(stringResource(R.string.revoke)) }
        },
        dismissButton = { WordAction(close) { Text(stringResource(R.string.cancel)) } },
    )
}
