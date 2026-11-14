import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: uso

    property string screen

    implicitWidth: row.implicitWidth + Look.e2 + Look.marcaTapa
    implicitHeight: Look.usoAlto
    radius: Look.radioChico
    color: Look.noche
    Accessible.role: Accessible.StaticText
    Accessible.name: words.text

    Row {
        id: row

        x: Look.e2
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.usoHueco

        IconImage {
            anchors.verticalCenter: parent.verticalCenter
            width: Look.icono
            height: Look.icono
            sourceSize.width: width
            sourceSize.height: height
            name: "check-circle-fill"
            color: Look.luna
        }

        Text {
            id: words

            anchors.verticalCenter: parent.verticalCenter
            text: uso.screen !== "" ? App.tr("On %1", App.screens(uso.screen.split(", "))) : App.tr("In use")
            font.family: Look.letraTexto
            font.pixelSize: Look.tTecla
            font.weight: Font.DemiBold
            color: Look.escarcha
        }
    }
}
