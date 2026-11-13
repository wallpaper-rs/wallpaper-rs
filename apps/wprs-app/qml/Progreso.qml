import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

GridLayout {
    id: progreso

    property string text
    property string count
    property real value: 0
    property bool indeterminate: false

    columns: 2
    rowSpacing: Look.listaHueco
    Accessible.role: Accessible.ProgressBar
    Accessible.name: text
    Accessible.description: count

    Text {
        Layout.fillWidth: true
        text: progreso.text
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    Text {
        text: progreso.count
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    QQC2.ProgressBar {
        Layout.columnSpan: 2
        Layout.fillWidth: true
        value: progreso.value
        indeterminate: progreso.indeterminate
        palette.dark: Look.escarcha
        palette.midlight: Look.nocheLinea
        Accessible.ignored: true
    }
}
