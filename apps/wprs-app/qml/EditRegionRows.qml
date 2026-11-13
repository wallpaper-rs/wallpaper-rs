import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: rows

    required property var page
    required property var zones

    function flip(order, on) {
        const rest = page.regionPicks.filter(o => o !== order);
        page.regionPicks = on ? rest.concat([order]).sort((a, b) => a - b) : rest;
    }

    spacing: 0

    Repeater {
        model: rows.zones

        delegate: QQC2.CheckDelegate {
            id: row

            required property var modelData

            Layout.fillWidth: true
            checked: rows.page.regionPicks.includes(modelData.order)
            text: App.tr("Region %1: depth %2, %3 % of the picture", modelData.order, modelData.depth.toFixed(2), (modelData.share * 100).toFixed(1))
            onToggled: rows.flip(modelData.order, checked)

            indicator: Rectangle {
                x: row.mirrored ? row.leftPadding : row.width - width - row.rightPadding
                y: row.topPadding + (row.availableHeight - height) / 2
                width: Kirigami.Units.gridUnit
                height: width
                radius: 2
                color: row.modelData.colour
                border.width: row.checked ? 3 : 1
                border.color: row.checked ? Kirigami.Theme.highlightColor : Kirigami.Theme.textColor
            }
        }
    }
}
