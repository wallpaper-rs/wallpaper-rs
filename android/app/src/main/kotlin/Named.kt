package io.github.wallpaper_rs

import android.content.Context
import android.net.Uri
import android.provider.MediaStore
import android.provider.OpenableColumns
import android.text.format.DateFormat
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.util.Locale

private val CODE = Regex("[\\p{L}\\p{N}_-]+")
private val WORD = Regex("\\p{L}{3,}")
private val STAMP = Regex("(?<!\\d)(20\\d{2})(\\d{2})(\\d{2})(?!\\d)")

fun coded(stem: String): Boolean = CODE.matches(stem) && !WORD.containsMatchIn(stem)

fun stamped(stem: String): LocalDate? = STAMP.find(stem)?.destructured?.let { (year, month, day) ->
    runCatching { LocalDate.of(year.toInt(), month.toInt(), day.toInt()) }.getOrNull()
}

fun readable(name: String, taken: Long?, words: (LocalDate?) -> String): String {
    val dot = name.lastIndexOf('.')
    val stem = if (dot > 0) name.substring(0, dot) else name
    if (!coded(stem.trim())) return name
    val date = taken?.takeIf { it > 0 }?.let { Instant.ofEpochMilli(it).atZone(ZoneId.systemDefault()).toLocalDate() } ?: stamped(stem)
    return words(date) + name.substring(stem.length)
}

private fun Context.column(uri: Uri, column: String): String? = runCatching {
    contentResolver.query(uri, arrayOf(column), null, null, null)?.use { row -> if (row.moveToFirst()) row.getString(0) else null }
}.getOrNull()

fun Context.titled(uri: Uri): String {
    val name = column(uri, OpenableColumns.DISPLAY_NAME).orEmpty()
    if (name.isEmpty()) return name
    val video = contentResolver.getType(uri)?.startsWith("video/") == true
    return readable(name, column(uri, MediaStore.MediaColumns.DATE_TAKEN)?.toLongOrNull()) { date ->
        val locale = resources.configuration.locales[0] ?: Locale.getDefault()
        val day = date?.format(DateTimeFormatter.ofPattern(DateFormat.getBestDateTimePattern(locale, "dMMMM"), locale))
        when {
            day == null -> getString(if (video) R.string.kind_video else R.string.kind_photo)
            else -> getString(if (video) R.string.video_dated else R.string.photo_dated, day)
        }
    }
}
