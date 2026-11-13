import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ApplicationWindow {
    id: window

    readonly property var args: Qt.application.arguments
    readonly property string sheet: args[args.indexOf("--pieces") + 1] ?? ""

    width: 1600
    height: 900
    visible: true
    visibility: Window.FullScreen
    color: Look.noche
    title: "wallpaper-rs"

    LookWindow {
        host: window
    }

    Loader {
        anchors.fill: parent
        focus: true
        source: /^[A-Za-z]+$/.test(window.sheet) ? "Pieces" + window.sheet[0].toUpperCase() + window.sheet.slice(1) + ".qml" : ""
    }
}
