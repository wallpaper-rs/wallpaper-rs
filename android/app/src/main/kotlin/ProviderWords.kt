package io.github.wallpaper_rs

import androidx.annotation.StringRes

@StringRes
fun kindTitle(kind: String) = when (kind) {
    "bing" -> R.string.kind_bing
    "apod" -> R.string.kind_apod
    "wikimedia" -> R.string.kind_wikimedia
    "wallhaven" -> R.string.kind_wallhaven
    "webdav" -> R.string.kind_webdav
    "nextcloud" -> R.string.kind_nextcloud
    else -> R.string.kind_immich
}

@StringRes
fun kindAbout(kind: String) = when (kind) {
    "bing" -> R.string.about_bing
    "apod" -> R.string.about_apod
    "wikimedia" -> R.string.about_wikimedia
    "wallhaven" -> R.string.about_wallhaven
    "webdav" -> R.string.about_webdav
    "nextcloud" -> R.string.about_nextcloud
    else -> R.string.about_immich
}

@StringRes
fun kindSent(kind: String) = when (kind) {
    "bing" -> R.string.sent_bing
    "apod" -> R.string.sent_apod
    "wikimedia" -> R.string.sent_wikimedia
    "wallhaven" -> R.string.sent_wallhaven
    "immich" -> R.string.sent_immich
    else -> R.string.sent_dav
}

@StringRes
fun optionLabel(key: String) = when (key) {
    "market" -> R.string.option_market
    "count" -> R.string.option_count
    "days" -> R.string.option_days
    "query" -> R.string.option_query
    "sorting" -> R.string.option_sorting
    "max_items" -> R.string.option_max_items
    "url" -> R.string.option_url
    "user" -> R.string.option_user
    "server" -> R.string.option_server
    "folder" -> R.string.option_folder
    "album" -> R.string.option_album
    "refresh" -> R.string.option_refresh
    else -> R.string.option_keep
}

@StringRes
fun valueLabel(key: String, value: String): Int? = when (key to value) {
    "refresh" to "6h" -> R.string.every_6h
    "refresh" to "12h" -> R.string.every_12h
    "refresh" to "1d" -> R.string.every_day
    "refresh" to "3d" -> R.string.every_3d
    "refresh" to "7d" -> R.string.every_week
    "sorting" to "date_added" -> R.string.sort_date_added
    "sorting" to "relevance" -> R.string.sort_relevance
    "sorting" to "random" -> R.string.sort_random
    "sorting" to "views" -> R.string.sort_views
    "sorting" to "favorites" -> R.string.sort_favorites
    "sorting" to "toplist" -> R.string.sort_toplist
    "sorting" to "hot" -> R.string.sort_hot
    else -> null
}

fun hostOf(address: String) = runCatching { android.net.Uri.parse(address.trim()).host }.getOrNull().orEmpty()
