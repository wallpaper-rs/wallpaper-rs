import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: missing

    property var report: ({})
    readonly property var lacking: report.rig ?? []

    visible: lacking.length > 0
    spacing: 0

    SceneRepeater {
        id: listed

        rows: missing.lacking

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: Look.ambar
            text: modelData.kind === "shape" ? App.tr("Line %1 names the blend shape %2, which %3 does not have", modelData.line, modelData.name, modelData.layers.join(", ")) : App.tr("Line %1 names the bone %2, which %3 does not have", modelData.line, modelData.name, modelData.layers.join(", "))
        }
    }
}
