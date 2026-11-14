package io.github.wallpaper_rs

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.produceState
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import org.json.JSONObject

typealias Run = (String, suspend () -> Unit) -> Unit

class Desk(val main: Main) {
    var task by mutableStateOf<String?>(null)
    var changes by mutableIntStateOf(0)
    var url by mutableStateOf("")
    var label by mutableStateOf("")
    var print by mutableStateOf<String?>(null)
    var scanned by mutableStateOf(false)
    var noCamera by mutableStateOf(false)
    var failed by mutableStateOf<Refused?>(null)
    var asked by mutableStateOf<String?>(null)
    var offer by mutableStateOf(false)
    var arrived by mutableStateOf<List<Item>?>(null)
    var plain by mutableStateOf(false)
    var choosing by mutableStateOf(false)
    var status: () -> JSONObject = { Sync.status() }

    val run: Run = { waiting, work ->
        main.act(false) {
            task = waiting
            try {
                work()
            } finally {
                task = null
                changes++
                Updates.settled(main)
            }
        }
    }

    fun connect(phone: String) {
        failed = null
        val pin = print
        run("") {
            try {
                if (Sync.status().optString("state") == "none") account("add", "url" to url.trim(), "fingerprint" to pin)
                account("connect", "label" to label.ifBlank { phone })
                account("wait")
                offer = true
            } catch (e: Refused) {
                if (e.id == OWN_CERTIFICATE) asked = e.args.firstOrNull() else failed = e
            }
        }
    }

    fun sync() {
        failed = null
        arrived = null
        val conflict = main.getString(R.string.conflict, "%1")
        run(main.getString(R.string.syncing)) {
            val before = fetch("items", "query" to "").map { it.uid }.toSet()
            try {
                account("sync", "conflict" to conflict)
            } catch (e: Refused) {
                failed = e
            }
            arrived = fetch("items", "query" to "").filter { it.uid !in before }.takeIf { it.isNotEmpty() }
        }
    }

    fun syncing() {
        offer = false
        failed = null
        run(main.getString(R.string.syncing)) {
            try {
                account("syncing", "on" to true, "plaintext" to plain)
            } catch (e: Refused) {
                failed = e
            }
        }
    }

    companion object {
        const val OWN_CERTIFICATE = "android.server_own_certificate"
        const val OTHER_CERTIFICATE = "android.server_other_certificate"
        const val UNREACHABLE = "android.server_unreachable"
        const val EXPIRED = "android.code_expired_before"
        val OFFLINE = setOf(UNREACHABLE, "android.sync_network")
    }
}

@Composable
fun watched(desk: Desk): State<JSONObject> {
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    return produceState(desk.status(), desk.changes, desk.task, lifecycle) {
        lifecycle.repeatOnLifecycle(Lifecycle.State.STARTED) {
            while (true) {
                value = withContext(Dispatchers.IO) { desk.status() }
                if (desk.task == null && value.isNull("link") && value.isNull("pairing")) break
                delay(1500)
            }
        }
    }
}

fun devices(status: JSONObject) =
    status.optJSONArray("devices")?.let { list -> List(list.length()) { list.getJSONObject(it) } }.orEmpty()

fun others(status: JSONObject) = devices(status).filterNot { it.optBoolean("current") }

fun JSONObject.text(key: String): String? = if (isNull(key)) null else getString(key)

fun JSONObject.server() = text("name")?.ifEmpty { null } ?: host(getString("url"))

fun host(url: String) = url.substringAfter("://").substringBefore('/').ifEmpty { url }

@Composable
fun ago(seconds: Long): String {
    val context = LocalContext.current
    val minutes = ((System.currentTimeMillis() / 1000 - seconds) / 60).coerceAtLeast(0)
    return when {
        minutes < 1 -> context.getString(R.string.just_now)
        minutes < 60 -> context.getString(R.string.minutes_ago, minutes)
        minutes < 60 * 24 -> context.getString(R.string.hours_ago, minutes / 60)
        else -> context.getString(R.string.days_ago, minutes / (60 * 24))
    }
}

@Composable
fun Waiting(main: Main, text: String) = Progress(text, null) { main.act(false) { account("cancel") } }
