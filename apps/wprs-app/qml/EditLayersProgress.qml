import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: progress

    required property var page
    readonly property var relief: page.relief
    readonly property var daemon: App.jobs.find(job => job.kind === "layers" && (job.state === "queued" || job.state === "running"))
    readonly property bool here: relief.busy && relief.stage === "building"
    readonly property real done: here ? relief.progress : daemon && daemon.total > 0 ? daemon.done / daemon.total : -1

    visible: here || daemon !== undefined

    RowLayout {
        Layout.fillWidth: true

        QQC2.Label {
            text: progress.here ? App.tr("Building the layers of this picture") : App.tr("The desktop is building the layers of a picture")
        }

        QQC2.Label {
            visible: progress.done >= 0
            text: Math.round(progress.done * 100) + " %"
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.ProgressBar {
            Layout.fillWidth: true
            from: 0
            to: 1
            indeterminate: progress.done < 0
            value: Math.max(0, progress.done)
        }

        QQC2.ToolButton {
            icon.name: "process-stop"
            text: App.tr("Cancel")
            visible: progress.here
            onClicked: progress.relief.cancel()
        }
    }
}
