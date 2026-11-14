import QtQuick
import QtQuick.Layouts
import QtQuick.Templates as T
import "../look"

T.AbstractButton {
    id: option

    property string glyph
    property string hint
    property string weight: "regular"
    property bool more: false
    property bool open: false
    readonly property color ink: enabled ? Look.escarcha : Look.brumaBaja

    implicitWidth: row.implicitWidth
    implicitHeight: Math.max(Look.altoOpcion, row.implicitHeight + Look.e3)
    leftPadding: Look.e4
    rightPadding: Look.e4
    focusPolicy: Qt.StrongFocus
    Accessible.name: text
    Accessible.role: Accessible.Button

    background: Rectangle {
        color: option.enabled && (option.hovered || option.visualFocus || option.down) ? Look.nocheAlta : "transparent"

        Ring {
            visible: option.visualFocus
            anchors.margins: Look.anillo
        }
    }

    contentItem: RowLayout {
        id: row

        spacing: Look.e3

        Glyph {
            name: option.glyph
            weight: option.weight
            tone: option.enabled ? Look.bruma : Look.brumaBaja
        }

        Words {
            Layout.fillWidth: true
            text: option.text
            color: option.ink
            font.weight: Font.Medium
            wrapMode: Text.Wrap
        }

        Words {
            visible: option.hint !== ""
            text: option.hint
            soft: true
            color: option.enabled ? Look.bruma : Look.brumaBaja
            font.pixelSize: Look.tPie
        }

        Glyph {
            visible: option.more
            name: option.open ? "caret-down" : "caret-right"
            tone: Look.bruma
        }
    }
}
