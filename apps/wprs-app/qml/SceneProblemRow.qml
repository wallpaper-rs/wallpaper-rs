import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var page
    required property var problem
    readonly property var owner: (page.view.layers ?? []).filter(item => problem.pointer === item.pointer || problem.pointer.startsWith(item.pointer + "/")).sort((a, b) => b.pointer.length - a.pointer.length)[0] ?? null
    readonly property string said: problem.message.replace(/^\/\S*:\s*/, "")
    readonly property string fixing: Object.keys(problem.fix ?? {})[0] ?? ""

    spacing: Look.e2
    Accessible.role: Accessible.AlertMessage
    Accessible.name: problem.message

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: 2
        name: row.problem.warning ? "warning" : "x-circle"
        sourceSize: Qt.size(Look.icono, Look.icono)
        color: row.problem.warning ? Look.ambar : Look.coral
    }

    ColumnLayout {
        Layout.fillWidth: true
        spacing: 0

        QQC2.Label {
            Layout.fillWidth: true
            text: row.owner ? App.tr("%1: %2", row.owner.id, row.said) : row.said
            wrapMode: Text.Wrap
        }

        Detalles {
            Layout.fillWidth: true
            details: row.problem.pointer
        }
    }

    QQC2.ToolButton {
        Layout.alignment: Qt.AlignTop
        text: App.tr("Go")
        onClicked: row.page.jump(row.problem.pointer)
    }

    QQC2.ToolButton {
        Layout.alignment: Qt.AlignTop
        visible: row.fixing !== ""
        text: App.tr("Fix")
        onClicked: row.page.run(row.fixing, row.problem.fix[row.fixing])
    }
}
