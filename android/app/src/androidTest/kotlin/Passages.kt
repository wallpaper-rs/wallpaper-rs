package io.github.wallpaper_rs

import android.graphics.Color
import java.io.File
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.assertTrue

object Passages {
    const val RED = 0xffff0000.toInt()
    const val BLUE = 0xff0000ff.toInt()
    const val GREEN = 0xff00ff00.toInt()

    fun install() {
        val root = File(storage(), "passages").apply { deleteRecursively() }
        SCENES.forEach { (name, scene) -> write(File(root, name), name, scene) }
        val reply = Native.library(JSONObject().put("op", "examples").put("dir", root.path).toString())
        assertTrue(reply, reply.startsWith("["))
    }

    fun pick(title: String) {
        val items = JSONArray(Native.library(JSONObject().put("op", "items").put("query", "kind:scene").toString()))
        val uid = (0 until items.length()).map(items::getJSONObject).first { it.getString("title") == title }.getString("uid")
        Native.library(JSONObject(mapOf("op" to "pick", "choice" to choice("item", uid))).toString())
    }

    fun marked(drawn: Int) = Color.red(drawn) > 0xc0 && Color.green(drawn) > 0xc0 && Color.blue(drawn) < 0x40

    fun mixed(drawn: Int) = Color.red(drawn) in 0x30..0xe0 && Color.blue(drawn) in 0x30..0xe0 && Color.green(drawn) < 0x30

    private fun write(dir: File, name: String, scene: String) {
        dir.mkdirs()
        File(dir, "scene.json").writeText(scene)
        File(dir, "manifest.json").writeText(
            """{ "schema": "wallpaper-rs.wpak/1", "id": "org.example.passage.${name.lowercase()}", "version": "1.0.0",
                "title": "$name", "license": "CC0-1.0", "kind": "scene", "entry": "scene.json" }""",
        )
        File(dir, "t").mkdirs()
        File(dir, "t/mark.json").writeText(MARK)
        File(dir, "t/mark.wgsl").writeText(if (name == "Broken") BROKEN else WGSL)
    }

    private const val MARK = """{ "id": "mark", "stage": "transition", "cost": { "taps": 1, "passes": 1, "scale": 1 } }"""
    private const val WGSL = """fn transition(c: Crossing) -> Crossing {
    var o = c;
    o.color = select(after(c.uv), vec4<f32>(1.0, 1.0, 0.0, 1.0), c.progress < 0.999);
    return o;
}
"""
    private const val BROKEN = """fn transition(c: Crossing) -> Crossing {
    return 1.0;
}
"""
    private fun scene(color: String, ms: Int?) = """{ "schema": "wallpaper-rs.scene/1", "canvas": { "size": [64, 64], "fit": "stretch" },
        ${ms?.let { """"transition": { "in": { "descriptor": "t/mark.json", "wgsl": "t/mark.wgsl", "duration_ms": $it } },""" } ?: ""}
        "layers": [{ "id": "fill", "type": "solid", "color": "$color" }] }"""
    private val SCENES = mapOf(
        "Red" to scene("#ff0000", null),
        "Mark" to scene("#0000ff", 1500),
        "Long" to scene("#0000ff", 6000),
        "Broken" to scene("#0000ff", 1500),
        "Green" to scene("#00ff00", null),
    )
}
