import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: words

    required property var preview
    readonly property var item: preview.candidate
    readonly property var plays: preview.plays
    readonly property string kindWord: !item ? "" : item.kind === "scene" ? App.tr("Scene") : item.kind === "video" ? App.tr("Video") : App.tr("Image")
    readonly property string still: !item ? "" : item.kind === "scene" ? App.tr("Still picture of the scene: it moves once applied") : item.kind === "video" ? App.tr("Still picture of the video: it moves once applied") : ""
    readonly property string seconds: (plays.duration_ms / 1000).toLocaleString(Qt.locale(), "f", 1)
    readonly property string reason: plays.why === "chosen" ? App.tr("chosen for this change") : plays.why === "screen" ? App.tr("the setting of this screen") : App.tr("your general setting")
    readonly property string how: plays.why === "calm" ? App.tr("simple fade · your system asks for less motion") : plays.why === "still" ? App.tr("simple fade · you asked for less motion") : plays.kind === "none" ? App.tr("no transition, %1", reason) : plays.kind === "own" ? App.tr("its author's transition, or a fade if it has none") : App.tr("fade · %1 s, %2", seconds, reason)
    readonly property string shortcut: String((App.state.options ? App.state.options["shortcuts.next"] : "") || "LOGO+ALT+N").replace(/LOGO/g, "Meta").replace(/CTRL/g, "Ctrl").replace(/ALT/g, "Alt").replace(/SHIFT/g, "Shift")
    readonly property var shownNow: preview.current
    readonly property string emptyText: !shownNow ? App.tr("Your library has no wallpapers yet. Add some and the shortcut %1 goes through them one by one, with their preview.", shortcut) : preview.session.source === "playlist" ? App.tr("The list «%1» has only «%2», which is already set. Add more wallpapers to it and the shortcut %3 goes through them one by one, with their preview.", preview.session.playlist_name, shownNow.title, shortcut) : App.tr("Your library has only «%1», which is already set. Add more wallpapers and the shortcut %2 goes through them one by one, with their preview.", shownNow.title, shortcut)

    width: PreviewLook.cambioTexto
    spacing: Look.e3

    Text {
        width: parent.width
        visible: words.preview.mode !== "vacio" && words.preview.mode !== "error"
        text: words.item ? words.item.title : ""
        elide: Text.ElideRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tGrande
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    Text {
        visible: words.preview.mode !== "vacio" && words.preview.mode !== "error"
        text: words.item && words.item.imported ? App.tr("%1 · My files", words.kindWord) : words.kindWord
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    Row {
        spacing: Look.e3
        visible: words.preview.mode !== "vacio" && words.preview.mode !== "error"

        Estado {
            anchors.verticalCenter: parent.verticalCenter
            text: words.preview.sampling ? App.tr("This is how it comes in: %1", words.how) : App.tr("Once applied: %1", words.how)
            icon: "sparkle"
        }

        QQC2.ProgressBar {
            anchors.verticalCenter: parent.verticalCenter
            width: PreviewLook.tira
            visible: words.preview.sampling
            value: words.preview.sample
        }
    }

    Nota {
        visible: words.still !== "" && words.preview.mode !== "vacio" && words.preview.mode !== "error"
        text: words.still
        icon: "image"
    }

    Vacio {
        width: parent.width
        visible: words.preview.mode === "vacio"
        title: App.tr("There is no other wallpaper to change to")
        text: words.emptyText
        action: App.tr("Open the library")
        actionIcon: "books"
        onActed: words.preview.finish(3)
    }

    Fallo {
        width: parent.width
        visible: words.preview.mode === "error"
        big: true
        title: App.tr("The preview could not open")
        text: App.tr("The wallpaper service does not answer. What you have set stays as it is.")
        details: words.preview.failure
        action: App.tr("Try again")
        onActed: words.preview.load()
    }
}
