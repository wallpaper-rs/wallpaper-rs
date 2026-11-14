import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: answer

    property string text
    property var value: undefined

    signal said(bool yes)

    spacing: Look.e2

    Text {
        Layout.fillWidth: true
        text: answer.text
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    QQC2.Button {
        text: App.tr("Yes")
        checkable: true
        checked: answer.value === true
        onClicked: answer.said(true)
    }

    QQC2.Button {
        text: App.tr("No")
        checkable: true
        checked: answer.value === false
        onClicked: answer.said(false)
    }
}
