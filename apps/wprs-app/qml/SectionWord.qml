import QtQuick
import QtQuick.Templates as T
import io.github.wallpaper_rs.app

T.AbstractButton {
    id: word

    required property bool current

    implicitWidth: label.implicitWidth
    implicitHeight: label.implicitHeight + Look.e3
    focusPolicy: Qt.TabFocus
    Accessible.role: Accessible.PageTab
    Accessible.name: text
    Accessible.selected: current
    Keys.onLeftPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
    Keys.onRightPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
    Keys.onReturnPressed: clicked()
    Keys.onEnterPressed: clicked()

    contentItem: Item {
        Text {
            id: label

            text: word.text
            color: word.current || word.hovered ? Look.escarcha : Look.bruma
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFiltro
            font.weight: word.current ? Font.Bold : Font.Medium
        }

        Rectangle {
            anchors.bottom: parent.bottom
            width: Look.marcaAncho
            height: Look.marcaAlto
            radius: 1
            color: Look.luna
            visible: word.current
        }

        Ring {
            visible: word.visualFocus
        }
    }
}
