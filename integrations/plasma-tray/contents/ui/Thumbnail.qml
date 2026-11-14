import QtQuick
import "../look"

Rectangle {
    id: thumbnail

    required property var daemon
    required property string item
    required property string source
    required property bool scene
    property string animated
    property bool moving

    radius: Look.radioChico
    color: Look.nocheAlta
    clip: true

    Image {
        anchors.fill: parent
        source: thumbnail.source
        asynchronous: true
        fillMode: Image.PreserveAspectCrop
    }

    AnimatedImage {
        anchors.fill: parent
        visible: thumbnail.animated !== "" && status === AnimatedImage.Ready
        source: thumbnail.animated
        playing: visible && thumbnail.moving && thumbnail.Window.visibility !== Window.Hidden
        cache: false
        fillMode: Image.PreserveAspectCrop
    }

    Glyph {
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: Look.e1
        width: Look.iconoNota
        height: width
        visible: thumbnail.scene
        name: "sparkle"
        tone: Look.escarcha
    }
}
