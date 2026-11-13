package io.github.wallpaper_rs

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class FailingMarkTest {
    private fun item(extra: String) = Item(JSONObject("""{"uid":"u","title":"t","path":"/p","kind":"image"$extra}"""))

    @Test
    fun aNullErrorIsNotAFailure() = assertNull(item(""","error":null""").failing)

    @Test
    fun aMissingErrorIsNotAFailure() = assertNull(item("").failing)

    @Test
    fun aTextErrorIsKept() = assertEquals("boom", item(""","error":"boom"""").failing)
}
