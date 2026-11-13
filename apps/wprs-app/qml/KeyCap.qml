import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: cap

    required property string key
    readonly property bool arrow: ["left", "right", "up", "down"].includes(key)

    implicitWidth: Math.max(Look.teclaAncho, word.implicitWidth + Look.e2 * 2)
    implicitHeight: Look.teclaAlto
    radius: Look.teclaRadio
    color: Look.nocheAlta
    border.width: Look.linea
    border.color: Look.nocheLinea
    Accessible.ignored: true

    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.leftMargin: cap.radius
        anchors.rightMargin: cap.radius
        height: Look.linea
        color: Look.nocheLinea
    }

    Kirigami.Icon {
        anchors.centerIn: parent
        width: Look.teclaIcono
        height: Look.teclaIcono
        visible: cap.arrow
        source: cap.key === "left" ? "arrow-left" : cap.key === "right" ? "arrow-right" : cap.key === "up" ? "arrow-up" : cap.key === "down" ? "arrow-down" : ""
    }

    Text {
        id: word

        anchors.centerIn: parent
        visible: !cap.arrow
        text: cap.arrow ? "" : cap.key
        color: Look.escarcha
        font.family: Look.letraTexto
        font.pixelSize: Look.tTecla
        font.weight: Font.DemiBold
    }
}
