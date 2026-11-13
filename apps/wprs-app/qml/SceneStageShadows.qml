import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: frusta

    required property var stage
    property var info: null
    readonly property var debug: JSON.parse(stage.page.editor.value("/scene/ext/editor/debug3d") || "null")
    readonly property bool shown: (debug?.show ?? debug) === "shadows"

    function load() {
        info = shown ? JSON.parse(stage.page.editor.rig("", JSON.stringify({
            "shadows3d": {
                "time": stage.page.editor.time
            }
        })) || "null") : null;
    }

    visible: shown
    onShownChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            frusta.load();
        }

        target: frusta.stage.page.editor
    }

    SceneRepeater {
        id: listed

        rows: frusta.info?.lights ?? []

        delegate: SceneLightLines {
            required property int index
            readonly property var modelData: listed.rows[index]

            anchors.fill: parent
            ratio: frusta.stage.ratio
            dashed: true
            tint: Look.coral
            segments: modelData.frustum ?? []
        }
    }
}
