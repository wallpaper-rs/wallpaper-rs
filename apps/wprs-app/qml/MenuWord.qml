import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: word

    required property int page
    required property bool apart
    required property bool current
    required property string text
    property bool lit: false
    readonly property bool hovered: hover.hovered

    signal clicked

    implicitWidth: Look.menuSangria + label.implicitWidth
    implicitHeight: apart ? Look.aparteAlto : Look.menuAlto
    Accessible.role: Accessible.PageTab
    Accessible.name: text
    Accessible.selected: current

    HoverHandler {
        id: hover

        cursorShape: Qt.PointingHandCursor
    }

    TapHandler {
        onTapped: word.clicked()
    }

    Item {
        anchors.fill: parent

        Rectangle {
            anchors.verticalCenter: parent.verticalCenter
            width: Look.marcaAncho
            height: Look.marcaAlto
            radius: 1
            color: Look.luna
            visible: word.current
        }

        Text {
            id: label

            anchors.verticalCenter: parent.verticalCenter
            x: Look.menuSangria
            text: word.text
            color: word.current || word.hovered || word.lit ? Look.escarcha : Look.bruma
            font.family: Look.letraTitulo
            font.pixelSize: word.apart ? Look.tAparte : Look.tMenu
            font.weight: word.current ? Font.Bold : word.apart ? Font.Medium : Font.Normal
            font.letterSpacing: -0.015 * (word.apart ? Look.tAparte : Look.tMenu)
        }

        Ring {
            anchors.leftMargin: Look.menuSangria - Look.e2 - Look.anilloHueco - Look.anillo
            anchors.rightMargin: -Look.e2 - Look.anilloHueco - Look.anillo
            visible: word.lit
        }
    }
}
