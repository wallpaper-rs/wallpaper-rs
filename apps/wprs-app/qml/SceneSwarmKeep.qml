import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

GridLayout {
    id: row

    required property var steer
    required property var keep
    readonly property bool ring: keep?.kind === "keep_distance"

    columns: 2

    QQC2.Label {
        Layout.columnSpan: 2
        text: row.ring ? App.tr("Keep a distance from") : App.tr("Keep between")
    }

    SceneRepeater {
        id: listed

        rows: row.ring ? [["target", row.keep?.target, row.keep?.at]] : [["from", row.keep?.from_ref, row.keep?.from], ["to", row.keep?.to_ref, row.keep?.to]]

        delegate: SceneTargetPicker {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            Layout.columnSpan: row.ring ? 2 : 1
            info: row.steer.tools.info
            value: modelData?.[1]
            at: modelData?.[2]
            onPicked: target => row.steer.patch(row.keep, v => v[modelData[0]] = target)
        }
    }

    QQC2.Label {
        visible: row.ring
        text: App.tr("Distance %1 px", Math.round(row.keep?.distance ?? 0))
    }

    SceneSlider {
        Layout.fillWidth: true
        visible: row.ring
        from: 0
        to: Math.max(1000, row.keep?.distance ?? 0)
        value: row.keep?.distance ?? 0
        onMoved: row.steer.patch(row.keep, v => v.distance = Math.round(value), true)
    }

    QQC2.Label {
        text: App.tr("Strength")
    }

    SceneSlider {
        Layout.fillWidth: true
        from: -50
        to: 50
        value: row.keep?.strength ?? 0
        onMoved: row.steer.patch(row.keep, v => v.strength = Math.round(value * 10) / 10, true)
    }

    QQC2.Button {
        Layout.columnSpan: 2
        icon.name: "edit-delete"
        text: App.tr("Remove this force")
        onClicked: row.steer.tools.edit("force", {
                "index": row.keep?.index,
                "remove": true
            })
    }
}
