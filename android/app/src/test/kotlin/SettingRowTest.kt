package io.github.wallpaper_rs

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class SettingRowTest {
    @Test
    fun aShortValueSitsBesideItsNameAndALongOneGoesUnder() {
        assertTrue(valueBeside("Automática"))
        assertTrue(valueBeside("Convertirlos"))
        assertFalse(valueBeside("Detalle primero: la mitad de fotogramas antes de bajar de nivel"))
        assertFalse(valueBeside(null))
    }
}
