import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Item {
    id: punto

    property url picture
    property string icon
    property string name
    property string note
    property string noteIcon
    property string noteTone
    property bool lit
    property bool chosen
    property bool off
    property bool grows
    readonly property alias hovered: area.containsMouse
    readonly property bool big: grows && lit
    readonly property bool marked: lit || chosen
    readonly property alias square: square

    signal tapped(int modifiers)
    signal doubleTapped

    implicitWidth: big ? Look.tapaCrearFoco : Look.tapaCrear
    implicitHeight: square.height + Look.e2 + label.height + (note !== "" ? words.height + Look.notaArriba : 0)
    Accessible.role: Accessible.ListItem
    Accessible.name: name
    Accessible.description: note
    Accessible.selected: lit

    Behavior on implicitWidth  {
        NumberAnimation {
            duration: Look.paso
            easing.type: Easing.BezierSpline
            easing.bezierCurve: Look.pasoCurva
        }
    }

    Rectangle {
        id: square

        width: parent.width
        height: Math.round(width * 9 / 16)
        radius: Look.radio
        color: Look.nocheAlta

        Image {
            anchors.fill: parent
            source: punto.picture
            asynchronous: true
            cache: true
            fillMode: Image.PreserveAspectCrop
            sourceSize.width: Look.tapaFoco
            sourceSize.height: Math.round(Look.tapaFoco * 9 / 16)
            opacity: punto.off ? Look.tapaApagada : 1
        }

        IconImage {
            anchors.centerIn: parent
            visible: punto.picture == "" && punto.icon !== ""
            name: punto.icon
            sourceSize: Qt.size(Look.icono * 2, Look.icono * 2)
            color: Look.bruma
        }

        Rectangle {
            anchors.fill: parent
            radius: Look.radio
            visible: !punto.marked
            color: Look.noche
            opacity: punto.hovered ? Look.veloTapaSobre : Look.veloTapa
        }

        Rectangle {
            visible: punto.lit
            anchors.fill: parent
            anchors.margins: -(Look.filete + Look.anillo)
            radius: Look.radio + Look.filete + Look.anillo
            color: "transparent"
            border.width: Look.anillo
            border.color: Look.escarcha
        }
    }

    Text {
        id: label

        y: square.height + Look.e2
        width: parent.width
        text: punto.name
        elide: Text.ElideRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tNombre
        font.weight: punto.marked ? Font.Bold : Font.Medium
        color: punto.marked ? Look.escarcha : punto.off ? Look.brumaBaja : Look.bruma
    }

    Row {
        id: words

        visible: punto.note !== ""
        y: label.y + label.height + Look.notaArriba
        width: parent.width
        spacing: Look.usoHueco

        IconImage {
            visible: punto.noteIcon !== ""
            y: 1
            width: Look.icono
            height: Look.icono
            sourceSize: Qt.size(Look.icono, Look.icono)
            name: punto.noteIcon
            color: Tono.of(punto.noteTone, false, Look.bruma)
        }

        Text {
            width: parent.width - (punto.noteIcon !== "" ? Look.icono + parent.spacing : 0)
            text: punto.note
            wrapMode: Text.WordWrap
            lineHeight: 1.4
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: punto.marked ? Look.escarcha : punto.noteTone !== "" ? Tono.of(punto.noteTone, false, Look.bruma) : Look.bruma
        }
    }

    MouseArea {
        id: area

        anchors.fill: square
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: mouse => punto.tapped(mouse.modifiers)
        onDoubleClicked: punto.doubleTapped()
    }
}
