package io.github.wallpaper_rs

const val MEDIA_SCENE = """{ "schema": "wallpaper-rs.scene/1", "canvas": { "size": [64, 64] },
    "layers": [{ "id": "back", "type": "solid", "color": "#000000" },
        { "id": "tint", "type": "solid", "color": { "media": "primary" }, "opacity": { "media": "playing" } }] }"""
const val MEDIA_MANIFEST = """{ "schema": "wallpaper-rs.wpak/2", "id": "org.example.tinted", "version": "1.0.0",
    "title": "Tinted", "license": "CC0-1.0", "kind": "scene", "entry": "scene.json", "origin": { "kind": "original" },
    "content": { "rating": "general" }, "photosensitive": false, "inputs": ["media"],
    "requires": { "features": ["inputs.media", "media_info"] } }"""
