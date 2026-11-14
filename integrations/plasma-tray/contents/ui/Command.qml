import QtQuick
import QtQuick.Layouts
import QtQuick.Templates as T
import "../look"

T.AbstractButton {
    id: command

    property string glyph
    property bool quiet: false
    property bool danger: false

    implicitWidth: row.implicitWidth + leftPadding + rightPadding
    implicitHeight: Look.altoControl
    leftPadding: Look.e3
    rightPadding: Look.e3
    focusPolicy: Qt.StrongFocus
    Accessible.name: text
    Accessible.role: Accessible.Button

    background: Rectangle {
        radius: Look.radio
        border.width: command.quiet ? 0 : Look.linea
        border.color: Look.nocheLinea
        color: !command.enabled ? "transparent" : command.down ? Look.noche : command.hovered || command.visualFocus ? Look.nocheLinea : command.quiet ? "transparent" : Look.nocheAlta

        Ring {
            visible: command.visualFocus
            corner: Look.radio
        }
    }

    contentItem: RowLayout {
        id: row

        spacing: Look.huecoBoton

        Glyph {
            visible: command.glyph !== ""
            name: command.glyph
            tone: command.enabled ? command.danger ? Look.coral : Look.bruma : Look.brumaBaja
        }

        Words {
            visible: command.text !== ""
            text: command.text
            font.weight: Font.Medium
            color: !command.enabled ? Look.brumaBaja : command.danger ? Look.coral : Look.escarcha
        }
    }
}
