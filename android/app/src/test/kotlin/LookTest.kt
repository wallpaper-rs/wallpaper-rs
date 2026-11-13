package io.github.wallpaper_rs

import java.io.File
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class LookTest {
    private val sources = listOf("src/main/kotlin", "app/src/main/kotlin").map(::File).first { it.isDirectory }

    private fun offending(pattern: Regex, except: Set<String> = emptySet()) = sources.walkTopDown()
        .filter { it.isFile && it.extension == "kt" && it.name !in except }
        .flatMap { file -> file.readLines().mapIndexedNotNull { n, line -> if (pattern.containsMatchIn(line)) "${file.name}:${n + 1}: ${line.trim()}" else null } }
        .toList()

    @Test
    fun theSourcesAreFound() {
        assertTrue(File(sources, "Tones.kt").isFile)
    }

    @Test
    fun noMaterialIconComesBack() {
        assertEquals(emptyList<String>(), offending(Regex("""androidx\.compose\.material\.icons|Icons\.(Default|Filled|Outlined|Rounded|Sharp|AutoMirrored)""")))
    }

    @Test
    fun colourLiteralsLiveOnlyInTheTones() {
        assertEquals(emptyList<String>(), offending(Regex("""Color\(0x|Color\.(White|Black|Red|Green|Blue|Gray|DarkGray|LightGray|Yellow|Cyan|Magenta)\b"""), setOf("Tones.kt")))
    }

    @Test
    fun everyGlyphHasItsRestAndLitDrawables() {
        val drawables = File(sources.parentFile, "res/drawable")
        val names = drawables.list().orEmpty().filter { it.startsWith("ph_") }.map { it.removeSuffix(".xml") }.toSet()
        assertTrue(names.size >= Ph.entries.size * 2)
        names.filterNot { it.endsWith("_fill") }.forEach { assertTrue(it, "${it}_fill" in names) }
    }
}
