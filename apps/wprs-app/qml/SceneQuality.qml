import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: quality

    required property var page
    property string again: App.tr("Restart")

    signal picked

    spacing: Look.e1

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Preview quality")
        wrapMode: Text.Wrap
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.ComboBox {
            Layout.fillWidth: true
            Layout.minimumWidth: 0
            Accessible.name: App.tr("Preview quality")
            model: SceneLook.names.map((name, i) => name + " · Q" + i)
            currentIndex: SceneLook.levels.indexOf(SceneLook.quality)
            onActivated: index => {
                SceneLook.quality = SceneLook.levels[index];
                SceneLook.preview(quality.page);
                quality.picked();
            }
        }

        QQC2.ToolButton {
            icon.name: "arrows-clockwise"
            text: quality.again
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: SceneLook.again(quality.page)
        }
    }
}
