import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: tasks

    property var dismissed: []
    readonly property var shown: App.jobs.filter(job => job.state === "queued" || job.state === "running" || (job.state === "failed" && dismissed.indexOf(job.id) < 0))

    spacing: Look.e1
    visible: shown.length > 0

    Repeater {
        model: tasks.shown

        delegate: Row {
            id: task

            required property var modelData
            readonly property bool failed: modelData.state === "failed"
            readonly property string what: modelData.kind === "analyze" ? App.tr("Making the thumbnails") : Labels.value(modelData.kind)

            anchors.right: parent ? parent.right : undefined
            spacing: Look.e2

            Estado {
                anchors.verticalCenter: parent.verticalCenter
                icon: task.failed ? "warning-circle" : "images"
                tone: task.failed ? "coral" : ""
                text: task.failed ? task.what + ": " + ((task.modelData.error && task.modelData.error.message) || App.tr("The job failed without giving a reason")) : task.modelData.total > 0 ? App.tr("%1: %2 of %3", task.what, task.modelData.done, task.modelData.total) : task.what
            }

            QQC2.Button {
                anchors.verticalCenter: parent.verticalCenter
                flat: true
                text: task.failed ? App.tr("Close") : App.tr("Cancel")
                onClicked: task.failed ? tasks.dismissed = tasks.dismissed.concat([task.modelData.id]) : App.act("CancelJob", [task.modelData.id])
            }
        }
    }
}
