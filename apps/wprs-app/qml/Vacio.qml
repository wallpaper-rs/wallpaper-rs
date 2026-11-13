import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: vacio

    property string title
    property string text
    property string action
    property string actionIcon
    property bool small: false

    signal acted

    width: Look.vacioAncho
    spacing: Look.e3
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Text {
        width: parent.width
        text: vacio.title
        wrapMode: Text.WordWrap
        lineHeight: 1.1
        font.family: Look.letraTitulo
        font.pixelSize: vacio.small ? Look.tFila : Look.tPagina
        font.weight: Font.Bold
        font.letterSpacing: vacio.small ? -0.015 * Look.tFila : -0.03 * Look.tPagina
        color: Look.escarcha
    }

    Text {
        width: Math.min(parent.width, 62 * font.pixelSize * 0.5)
        visible: vacio.text !== ""
        text: vacio.text
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.letraTexto
        font.pixelSize: vacio.small ? Look.tCuerpo : Look.tExplica
        color: Look.bruma
    }

    Item {
        width: 1
        height: Look.e3
        visible: vacio.action !== ""
    }

    QQC2.Button {
        visible: vacio.action !== ""
        highlighted: true
        text: vacio.action
        icon.name: vacio.actionIcon
        onClicked: vacio.acted()
    }
}
