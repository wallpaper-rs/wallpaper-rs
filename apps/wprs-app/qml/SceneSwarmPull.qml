import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

GridLayout {
    id: row

    required property var steer
    required property var pull
    readonly property var value: pull.ref

    columns: 2

    QQC2.Label {
        text: SceneWords.value(row.pull.kind)
    }

    SceneTargetPicker {
        Layout.fillWidth: true
        info: row.steer.tools.info
        value: row.value
        at: row.pull.at
        onPicked: target => row.steer.patch(row.pull, v => v[row.pull.field] = target)
    }

    QQC2.Label {
        text: row.pull.kind === "slow_near" ? App.tr("Speed kept %1", Math.round((row.pull.factor ?? 0) * 100) / 100) : App.tr("Strength")
    }

    SceneSlider {
        Layout.fillWidth: true
        readonly property bool kept: row.pull.kind === "slow_near"

        from: kept ? 0 : -500
        to: kept ? 1 : 500
        value: kept ? (row.pull.factor ?? 0) : (row.pull.strength ?? 0)
        onMoved: row.steer.patch(row.pull, v => {
                if (kept)
                    v.factor = Math.round(value * 100) / 100;
                else
                    v.strength = Math.round(value);
            }, true)
    }

    SceneRepeater {
        id: listed

        rows: row.pull.rings

        delegate: RowLayout {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.columnSpan: 2

            QQC2.Label {
                text: App.tr("%1: %2 px", SceneWords.label(modelData.key), Math.round(modelData.value))
            }

            SceneSlider {
                Layout.fillWidth: true
                from: 0
                to: Math.max(1000, modelData.value)
                value: modelData.value
                onMoved: row.steer.patch(row.pull, v => {
                        if (modelData.key === "ring")
                            v.ring.radius = Math.round(value);
                        else
                            v[modelData.key] = Math.round(value);
                    }, true)
            }
        }
    }

    QQC2.Button {
        Layout.columnSpan: 2
        icon.name: "edit-delete"
        text: App.tr("Remove this force")
        onClicked: row.steer.tools.edit("force", {
                "index": row.pull.index,
                "remove": true
            })
    }
}
