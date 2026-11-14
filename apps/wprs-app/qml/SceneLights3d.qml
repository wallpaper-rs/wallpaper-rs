import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: placed

    required property var page
    property var info: null
    readonly property var lights: info?.lights ?? []

    function load() {
        info = JSON.parse(page.editor.rig("", JSON.stringify({
            "stage3d": {
                "time": page.editor.time
            }
        })) || "null");
    }

    visible: lights.length > 0 && (info?.stages ?? 0) > 0
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            placed.load();
        }

        target: placed.page.editor
    }

    Kirigami.Heading {
        level: 5
        text: App.tr("Lights in 3D")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("A light in 3D stands at its own point among the models and a spot points at its aim; on the plane it follows the light layer and its height. Drag its knobs in the authoring view, with Ctrl along the line of sight.")
    }

    SceneRepeater {
        id: listed

        rows: placed.lights

        delegate: QQC2.CheckBox {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            text: App.tr("%1 in 3D", modelData.id)
            checked: modelData.space
            onToggled: placed.page.run("scene3d", {
                "light": checked ? {
                    "layer": modelData.id,
                    "place": true
                } : {
                    "layer": modelData.id,
                    "flat": true
                }
            }, true)
        }
    }
}
