import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var scene: [
        {
            "id": "empty",
            "icon": "stack",
            "name": App.tr("Empty scene"),
            "note": App.tr("Start from nothing and add the layers you want.")
        },
        {
            "id": "particles",
            "icon": "sparkle",
            "picture": "qrc:/wprs/starters/particles.jpg",
            "name": App.tr("Particles"),
            "note": App.tr("Points that are born, float and fade out.")
        },
        {
            "id": "parallax",
            "icon": "stack",
            "picture": "qrc:/wprs/starters/parallax.jpg",
            "name": App.tr("Layered parallax"),
            "note": App.tr("Layers at different depths, with relief.")
        },
        {
            "id": "clock",
            "icon": "clock",
            "picture": "qrc:/wprs/starters/clock.jpg",
            "name": App.tr("Text clock"),
            "note": App.tr("The time or the date, with your own font and colors.")
        }
    ]
    readonly property var single: [
        {
            "id": "image",
            "icon": "image",
            "name": App.tr("Picture"),
            "note": App.tr("One still picture. To add text or effects, use “Empty scene”.")
        },
        {
            "id": "video",
            "icon": "file-video",
            "name": App.tr("Video"),
            "note": App.tr("A video that loops without end.")
        },
        {
            "id": "web",
            "icon": "globe",
            "name": App.tr("Web page"),
            "note": App.tr("A page of your own that moves as the wallpaper.")
        }
    ]
    readonly property var more: [
        {
            "id": "preset",
            "icon": "sliders-horizontal",
            "name": App.tr("Preset"),
            "note": App.tr("A saved set of settings for a wallpaper you already have.")
        },
        {
            "id": "assets",
            "icon": "package",
            "name": App.tr("Resource pack"),
            "note": App.tr("Parts of a scene, ready to use in other scenes.")
        }
    ]
}
