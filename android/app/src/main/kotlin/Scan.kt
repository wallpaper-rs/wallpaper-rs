package io.github.wallpaper_rs

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.core.content.ContextCompat
import org.json.JSONObject
import java.util.concurrent.atomic.AtomicBoolean

@Composable
fun Scan(lens: Lens, found: (JSONObject) -> Unit, blind: (Int) -> Unit, close: () -> Unit) {
    val context = LocalContext.current
    val present = remember { lens.present(context) }
    var allowed by remember { mutableStateOf(lens.allowed(context)) }
    var line by remember { mutableStateOf<String?>(null) }
    val taken = remember { AtomicBoolean(false) }
    val ask = lens.asker { if (it) allowed = true else blind(R.string.camera_refused) }
    LaunchedEffect(Unit) {
        when {
            !present -> blind(R.string.camera_missing)
            !allowed -> ask()
        }
    }
    val main = remember { ContextCompat.getMainExecutor(context) }
    TaskPage(foot = null) {
        Header(stringResource(R.string.scan_code), close)
        Text(stringResource(R.string.scan_intro), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2), tones.mist, style = Type.text)
        if (present && allowed) {
            lens.View(Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2).fillMaxWidth().clip(RoundedCornerShape(Measure.rNotice)).testTag("lens")) { frame, size ->
                if (taken.get()) return@View
                val text = Native.readCode(frame, size)
                if (text.isEmpty()) return@View
                val reply = JSONObject(Native.connectCode(text))
                val refused = refusal(reply)
                if (refused == null && !taken.compareAndSet(false, true)) return@View
                main.execute { if (refused == null) found(reply) else line = context.told(refused) }
            }
        }
        line?.let { StateLine(it, Tone.Coral, Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2).testTag("scan-line")) }
    }
}

fun Desk.take(code: JSONObject, phone: String) {
    url = code.getString("server")
    print = code.text("fingerprint")
    scanned = true
    blind = null
    dial(phone)
}
