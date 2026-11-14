import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: tapa

    property url picture
    property string name
    property string note
    property string noteIcon
    property string noteTone
    property bool lit
    property bool off
    property bool inUse
    property bool choosing
    property bool chosen
    property bool even
    readonly property alias hovered: area.containsMouse
    default property alias below: under.data

    signal tapped(int modifiers)
    signal doubleTapped

    readonly property bool grown: lit && !even

    implicitWidth: grown ? Look.tapaFoco : Look.tapa
    implicitHeight: square.height + (grown ? under.height + Look.e4 : label.height + Look.e3 + (note ? caption.height + Look.notaArriba : 0))
    Accessible.role: Accessible.ListItem
    Accessible.name: name
    Accessible.description: note
    Accessible.selected: lit
    Accessible.checkable: choosing
    Accessible.checked: chosen

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

        Sombra {
            kind: tapa.lit ? "foco" : "tapa"
        }

        Foto {
            anchors.fill: parent
            source: tapa.picture
            sourceSize.width: Look.tapaFoco
            sourceSize.height: Math.round(Look.tapaFoco * 9 / 16)
            opacity: tapa.off ? Look.tapaApagada : 1
        }

        Rectangle {
            anchors.fill: parent
            radius: Look.radio
            visible: !tapa.lit
            color: Look.noche
            opacity: tapa.chosen ? Look.veloTapaElegida : tapa.hovered ? Look.veloTapaSobre : Look.veloTapa
        }

        Anillo {
            visible: tapa.lit
            gap: Look.filete
        }

        Uso {
            visible: tapa.inUse
            x: Look.marcaTapa
            y: parent.height - height - Look.marcaTapa
        }

        Elegir {
            visible: tapa.choosing
            x: Look.marcaTapa
            y: Look.marcaTapa
            chosen: tapa.chosen
        }
    }

    Text {
        id: label

        visible: !tapa.grown
        y: square.height + Look.e3
        width: parent.width
        text: tapa.name
        elide: Text.ElideRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tNombre
        font.weight: tapa.lit ? Font.Bold : Font.Medium
        color: tapa.lit ? Look.escarcha : Look.bruma
    }

    Nota {
        id: caption

        visible: !tapa.grown && tapa.note !== ""
        y: label.y + label.height + Look.notaArriba
        fitWidth: parent.width
        text: tapa.note
        tone: tapa.noteTone
        icon: tapa.noteIcon
    }

    Item {
        id: under

        visible: tapa.grown
        y: square.height + Look.e4
        width: childrenRect.width
        height: childrenRect.height
    }

    MouseArea {
        id: area

        anchors.fill: square
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: mouse => tapa.tapped(mouse.modifiers)
        onDoubleClicked: tapa.doubleTapped()
    }
}
