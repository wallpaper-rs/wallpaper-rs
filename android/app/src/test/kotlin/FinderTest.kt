package io.github.wallpaper_rs

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class FinderTest {
    @Test
    fun theSearchIgnoresCaseAndAccents() {
        assertTrue(finds("bateria", "Pausar con batería"))
        assertTrue(finds("  PAUSAR ", "Pausar con batería"))
        assertTrue(finds("", "Calidad"))
        assertFalse(finds("zzqx", "Calidad"))
    }
}
