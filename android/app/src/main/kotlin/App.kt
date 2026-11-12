package io.github.wallpaper_rs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.List
import androidx.compose.material.icons.filled.AccountCircle
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Badge
import androidx.compose.material3.BadgedBox
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import java.io.File
import androidx.compose.ui.viewinterop.AndroidView
import android.widget.ImageView
import android.graphics.drawable.AnimatedImageDrawable
import android.graphics.ImageDecoder

private val TABS = listOf(
    R.string.library to Icons.Default.Home,
    R.string.playlists to Icons.AutoMirrored.Filled.List,
    R.string.sources to Icons.Default.Search,
    R.string.devices to Icons.Default.AccountCircle,
    R.string.settings to Icons.Default.Settings,
)

@Composable
fun App(main: Main) {
    val context = LocalContext.current
    val colors = if (isSystemInDarkTheme()) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
    var tab by rememberSaveable { mutableIntStateOf(0) }
    var item by rememberSaveable { mutableStateOf<String?>(null) }
    var playlist by rememberSaveable { mutableStateOf<String?>(null) }
    BackHandler(item != null || playlist != null || tab != 0) {
        when {
            item != null -> item = null
            playlist != null -> playlist = null
            else -> tab = 0
        }
    }
    MaterialTheme(colorScheme = colors) {
        if (main.placing.value) Placing(main)
        Scaffold(bottomBar = {
            if (item == null) NavigationBar {
                TABS.forEachIndexed { index, (label, icon) ->
                    NavigationBarItem(
                        selected = tab == index,
                        onClick = {
                            tab = index
                            playlist = null
                        },
                        icon = {
                            val updates = Updates.count.intValue
                            if (label != R.string.sources || updates == 0) Icon(icon, null)
                            else BadgedBox({ Badge { Text("$updates") } }) { Icon(icon, stringResource(R.string.repo_updates)) }
                        },
                        label = { Text(stringResource(label)) },
                    )
                }
            }
        }) { padding ->
            Box(Modifier.padding(padding)) {
                val (shown, list) = item to playlist
                when {
                    shown != null -> Detail(main, shown, { item = it }) { item = null }
                    tab == 0 -> Library(main, { tab = SOURCES }) { item = it }
                    tab == 1 && list != null -> Entries(main, list, { item = it }) { playlist = null }
                    tab == 1 -> Playlists(main) { playlist = it }
                    tab == SOURCES -> Sources(main) {
                        main.search.value = it
                        tab = 0
                    }
                    tab == 3 -> Account(main)
                    else -> Settings(main)
                }
            }
        }
    }
}

@Composable
fun Header(title: String, back: (() -> Unit)? = null) = Row(verticalAlignment = Alignment.CenterVertically) {
    back?.let { IconButton(it) { Icon(Icons.AutoMirrored.Filled.ArrowBack, stringResource(R.string.back)) } }
    Text(title, Modifier.padding(16.dp), style = MaterialTheme.typography.headlineSmall)
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Choices(label: String?, options: List<Pair<String, String>>, chosen: String, choose: (String) -> Unit) =
    Column(Modifier.padding(horizontal = 16.dp, vertical = 8.dp)) {
        label?.let { Text(it, style = MaterialTheme.typography.titleSmall) }
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            options.forEach { (value, text) -> FilterChip(chosen == value, { choose(value) }, { Text(text) }) }
        }
    }

@Composable
fun Toggle(label: String, checked: Boolean, enabled: Boolean = true, change: (Boolean) -> Unit) = ListItem(
    headlineContent = { Text(label) },
    trailingContent = { Switch(checked, null, enabled = enabled) },
    modifier = Modifier.toggleable(value = checked, enabled = enabled, role = Role.Switch, onValueChange = change),
)

@Composable
fun Note(text: String, color: Color = Color.Unspecified) =
    Text(text, Modifier.padding(16.dp, 4.dp), color, style = MaterialTheme.typography.bodySmall)

@Composable
fun Animated(path: String, modifier: Modifier = Modifier) = AndroidView({ ImageView(it) }, modifier) { view ->
    if (view.tag != path) {
        view.tag = path
        val drawable = runCatching { ImageDecoder.decodeDrawable(ImageDecoder.createSource(File(path))) }.getOrNull()
        view.setImageDrawable(drawable)
        (drawable as? AnimatedImageDrawable)?.start()
    }
}

private const val SOURCES = 2
