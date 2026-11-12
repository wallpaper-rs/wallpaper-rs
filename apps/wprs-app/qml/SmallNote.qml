import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.Label {
    Layout.fillWidth: true
    wrapMode: Text.Wrap
    textFormat: Text.PlainText
    color: Kirigami.Theme.disabledTextColor
    font: Kirigami.Theme.smallFont
}
