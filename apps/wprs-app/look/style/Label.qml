import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.Label {
    id: control

    readonly property bool heading: "level" in control

    font.family: heading ? Look.letraTitulo : Look.letraTexto
    color: enabled ? Look.escarcha : Look.brumaBaja
    linkColor: Look.luna
}
