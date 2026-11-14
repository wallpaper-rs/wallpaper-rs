import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    property var finding: ({})
    property bool first: false
    readonly property string level: finding.level ?? "note"

    signal go

    spacing: Look.e3
    Accessible.role: Accessible.StaticText
    Accessible.name: row.finding.message ?? ""

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: Look.e1
        name: row.level === "error" ? "x-circle" : row.level === "warning" ? "warning" : "info"
        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
        color: row.level === "error" ? Look.coral : row.level === "warning" ? Look.ambar : Look.bruma
    }

    Text {
        Layout.fillWidth: true
        text: row.finding.message ?? ""
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    QQC2.Button {
        Layout.alignment: Qt.AlignTop
        flat: true
        visible: (row.finding.pointer ?? "") !== ""
        text: App.tr("Go to the problem")
        onClicked: row.go()
    }
}
