package io.github.wallpaper_rs

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType

private const val PRINT = 64

fun printed(text: String) = text.filter { it.isLetterOrDigit() }.lowercase()

@Composable
fun Print(desk: Desk, close: () -> Unit) {
    val status by watched(desk)
    val context = LocalContext.current
    var typed by rememberSaveable { mutableStateOf(desk.print.orEmpty()) }
    var checked by rememberSaveable { mutableStateOf<Boolean?>(null) }
    val print = printed(typed)
    val linked = status.optString("state") !in setOf("none", "linking", "unlinked")
    val problem = when {
        print.isEmpty() -> null
        print.any { it !in "0123456789abcdef" } -> stringResource(R.string.print_not_hex)
        print.length < PRINT -> stringResource(R.string.print_short, PRINT - print.length)
        print.length > PRINT -> stringResource(R.string.print_long)
        checked == false -> stringResource(R.string.print_differs)
        else -> null
    }
    val phone = stringResource(R.string.this_phone)
    TaskPage(foot = {
        Action({
            if (linked) {
                checked = printed(status.optString("pin")) == print
            } else {
                desk.print = print
                desk.failed = null
                desk.connect(phone)
                close()
            }
        }, Modifier.weight(1f).testTag("check-print"), print.length == PRINT && problem == null) {
            Glyph(Ph.CheckCircle, null)
            Text(stringResource(R.string.check))
        }
    }) {
        Header(stringResource(R.string.print_row), close)
        Text(stringResource(R.string.print_intro), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), tones.mist, style = Type.text)
        TextBox(
            stringResource(R.string.print_field),
            typed,
            { typed = it; checked = null },
            Modifier.testTag("print"),
            problem = problem,
            kind = KeyboardType.Ascii,
        ) {
            Square({ context.pasted().takeIf { it.isNotEmpty() }?.let { typed = it; checked = null } }, Ph.ClipboardText, stringResource(R.string.paste), boxed = false)
        }
        if (checked == true) StateLine(stringResource(R.string.print_same), Tone.Sprout, Modifier.padding(horizontal = Measure.margin))
    }
}
