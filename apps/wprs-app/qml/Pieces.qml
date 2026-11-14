import QtQuick
import io.github.wallpaper_rs.app

Window {
    id: window

    readonly property var args: Qt.application.arguments
    readonly property string sheet: args[args.indexOf("--pieces") + 1] ?? ""

    width: 1600
    height: 900
    visible: true
    visibility: window.sheet.toLowerCase() === "band" ? Window.Windowed : Window.FullScreen
    color: Look.noche
    title: "wallpaper-rs"

    LookWindow {
        host: window
        tight: !!(loaded.item && loaded.item.frame && loaded.item.frame.tight)
    }

    Loader {
        id: loaded

        anchors.fill: parent
        focus: true
        source: /^[A-Za-z]+$/.test(window.sheet) ? "Pieces" + window.sheet[0].toUpperCase() + window.sheet.slice(1) + ".qml" : ""
    }
}
