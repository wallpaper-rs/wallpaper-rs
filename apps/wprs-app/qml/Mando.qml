import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Item {
    id: mando

    property string text
    property string icon
    property bool active: false
    property bool off: false
    property bool first: false
    readonly property bool hovered: hover.hovered
    default property alias extra: more.data

    signal clicked

    implicitWidth: row.implicitWidth + Look.e4 * 2 - Look.e1
    implicitHeight: Look.altoControl
    activeFocusOnTab: !off
    Accessible.role: Accessible.Button
    Accessible.name: text
    Keys.onReturnPressed: !off && clicked()
    Keys.onEnterPressed: !off && clicked()
    Keys.onSpacePressed: !off && clicked()

    HoverHandler {
        id: hover

        enabled: !mando.off
        cursorShape: Qt.PointingHandCursor
    }

    TapHandler {
        id: tap

        enabled: !mando.off
        onTapped: mando.clicked()
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radio
        color: tap.pressed ? Look.nocheLinea : mando.hovered ? Look.nocheAlta : "transparent"
    }

    Rectangle {
        visible: !mando.first
        width: Look.linea
        height: parent.height
        color: Look.nocheLinea
    }

    Row {
        id: row

        x: Look.e4 - Look.e1
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.e2

        IconImage {
            visible: mando.icon !== ""
            anchors.verticalCenter: parent.verticalCenter
            name: mando.icon
            sourceSize: Qt.size(Look.icono, Look.icono)
            color: mando.off ? Look.brumaBaja : mando.active ? Look.escarcha : Look.bruma
        }

        Text {
            visible: mando.text !== "" && more.children.length === 0
            anchors.verticalCenter: parent.verticalCenter
            text: mando.text
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: Font.Medium
            color: mando.off ? Look.brumaBaja : Look.escarcha
        }

        Row {
            id: more

            anchors.verticalCenter: parent.verticalCenter
            spacing: Look.e2
        }
    }

    Anillo {
        visible: mando.activeFocus
        gap: 0
    }
}
