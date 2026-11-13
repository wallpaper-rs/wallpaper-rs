import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Item {
    id: opcion

    property string text
    property string icon
    property string note
    property string tone
    property bool off: false
    property bool lit: false
    readonly property bool hovered: hover.hovered
    readonly property color ink: off ? Look.brumaBaja : lit ? Tono.of(tone, true, Look.noche) : Tono.of(tone, false, Look.escarcha)

    signal clicked

    implicitHeight: Look.altoOpcion
    Accessible.role: Accessible.MenuItem
    Accessible.name: note !== "" ? text + ", " + note : text
    Accessible.focused: lit

    HoverHandler {
        id: hover
    }

    TapHandler {
        enabled: !opcion.off
        onTapped: opcion.clicked()
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radioChico
        color: opcion.lit && !opcion.off ? Look.escarcha : opcion.hovered && !opcion.off ? Look.nocheAlta : "transparent"
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e3

        IconImage {
            visible: opcion.icon !== ""
            name: opcion.icon
            sourceSize: Qt.size(Look.icono, Look.icono)
            color: opcion.off ? Look.brumaBaja : opcion.lit ? opcion.ink : Tono.of(opcion.tone, false, Look.bruma)
        }

        Text {
            Layout.fillWidth: true
            text: opcion.text
            elide: Text.ElideRight
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: opcion.lit && !opcion.off ? Font.DemiBold : Font.Normal
            color: opcion.ink
        }

        Text {
            visible: opcion.note !== ""
            text: opcion.note
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: opcion.off ? Look.brumaBaja : opcion.lit ? Look.noche : Look.bruma
        }
    }
}
