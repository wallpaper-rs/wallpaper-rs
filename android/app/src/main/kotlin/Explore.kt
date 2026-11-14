package io.github.wallpaper_rs

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import org.json.JSONArray
import org.json.JSONObject

private val COVER = 150.dp

@Composable
fun Explore(main: Main, back: () -> Unit, manage: () -> Unit, connect: () -> Unit) {
    var query by rememberSaveable { mutableStateOf("") }
    var installed by remember { mutableIntStateOf(0) }
    val revision = main.revision.intValue
    val repos by loaded(JSONObject(), revision, installed) { account("repos") as? JSONObject ?: JSONObject() }
    val found by loaded(JSONArray(), query, installed, revision) { account("browse", "query" to query) as? JSONArray ?: JSONArray() }
    val status by loaded(JSONObject(), revision) { Sync.status() }
    val shelves = repos.optJSONArray("repos").objects().map { repo -> repo to found.objects().filter { it.getString("repo_id") == repo.getString("repo_id") } }
    val install = { item: JSONObject ->
        main.act(false) {
            main.complete(account("install", "repo_id" to item.getString("repo_id"), "id" to item.getString("id")))
            installed++
        }
        Unit
    }
    LazyColumn(Modifier.testTag("explore_page")) {
        item(key = "header") { Header(stringResource(R.string.repo_browse), back) }
        item(key = "search") { Seek(query, { query = it }, stringResource(R.string.repo_search_all), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2)) }
        shelves.filter { it.second.isNotEmpty() }.forEach { (repo, packages) ->
            item(key = repo.getString("repo_id")) { Packages(repo, packages, manage, install) }
        }
        if (shelves.all { it.second.isEmpty() }) item(key = "nothing") { Note(stringResource(R.string.repo_nothing)) }
        if (linkedName(status, true) == null) item(key = "server") {
            Column {
                Group(stringResource(R.string.your_server))
                Setting(stringResource(R.string.connect_block), Modifier.testTag("connect_server"), stringResource(R.string.connect_server_note), onClick = connect) {
                    Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
                }
            }
        }
        item(key = "manage") {
            Setting(stringResource(R.string.sources), Modifier.testTag("manage_repos"), stringResource(R.string.repo_manage_note), onClick = manage) {
                Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
            }
        }
        item(key = "end") { Spacer(Modifier.height(Measure.e8)) }
    }
}

@Composable
private fun Packages(repo: JSONObject, packages: List<JSONObject>, manage: () -> Unit, install: (JSONObject) -> Unit) = Column(Modifier.padding(top = Measure.e3)) {
    val bundled = repo.optBoolean("bundled")
    Row(
        Modifier.fillMaxWidth().clickable(role = Role.Button, onClick = manage).padding(start = Measure.margin, end = Measure.e3, top = Measure.e2, bottom = Measure.e2),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Column(Modifier.weight(1f)) {
            Text(repoName(bundled, repo.getString("name")), style = Type.row)
            Text(if (bundled) stringResource(R.string.repo_on_phone) else stringResource(R.string.repo_packages, repo.optInt("packages")), color = tones.mist, style = Type.foot)
        }
        Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
    }
    LazyRow(contentPadding = PaddingValues(horizontal = Measure.margin, vertical = Measure.e2), horizontalArrangement = Arrangement.spacedBy(Measure.e3)) {
        items(packages, key = { it.getString("id") }) { item -> Offered(item, install) }
    }
}

@Composable
private fun Offered(item: JSONObject, install: (JSONObject) -> Unit) = Column(Modifier.width(COVER).testTag("offered")) {
    val picture by preview(item.getString("repo_id") + "/" + item.getString("id") + "/" + item.getString("version"))
    val frame = Modifier.fillMaxWidth().aspectRatio(16f / 10f).clip(RoundedCornerShape(Measure.rControl))
    picture?.let { Image(it, null, frame, contentScale = ContentScale.Crop) } ?: Column(frame.background(tones.nightHigh)) {}
    Text(item.optString("title").ifEmpty { item.getString("id") }, Modifier.padding(top = Measure.e1), style = Type.foot, maxLines = 1, overflow = TextOverflow.Ellipsis)
    if (item.optBoolean("mature")) Text(stringResource(R.string.mature), color = tones.amber, style = Type.foot)
    if (item.optBoolean("installed")) Text(stringResource(R.string.repo_installed), color = tones.mist, style = Type.foot)
    else WordAction({ install(item) }) { Text(stringResource(R.string.repo_install)) }
}
