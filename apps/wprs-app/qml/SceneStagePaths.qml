import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: paths

    required property var stage
    property var info: null
    readonly property var handles: stage.info?.handles ?? null

    function load() {
        info = JSON.parse(stage.page.editor.rig("", JSON.stringify({
                        "paths3d": {
                            "time": stage.page.editor.time
                        }
                    })) || "null");
    }

    function moveKey(path, key, aim, to, held) {
        stage.grab({
            "key": {
                "path": path,
                "key": key.t,
                "aim": aim
            }
        }, held & Qt.ControlModifier ? "depth" : "plane", aim ? key.aim : key.at, to);
    }

    visible: handles !== null && (info?.paths ?? []).length > 0
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            paths.load();
        }

        function onTimeChanged() {
            paths.load();
        }

        target: paths.stage.page.editor
    }

    SceneRepeater {
        id: listed3

        rows: paths.info?.paths ?? []

        delegate: Item {
            id: one

            required property int index
            readonly property var modelData: listed3.rows[index]

            anchors.fill: parent

            SceneLightLines {
                anchors.fill: parent
                ratio: paths.stage.ratio
                tint: Kirigami.Theme.positiveTextColor
                segments: one.modelData.spline.slice(1).map((p, i) => [one.modelData.spline[i], p])
            }

            SceneRepeater {
                id: listed2

                rows: one.modelData.keys.filter(k => k.at)

                delegate: SceneKnob {
                    required property int index
                    readonly property var modelData: listed2.rows[index]

                    at: modelData.at
                    ratio: paths.stage.ratio
                    label: App.tr("%1 at %2 s", one.modelData.id, modelData.t)
                    tint: Kirigami.Theme.positiveTextColor
                    tip: App.tr("Drag to move this key of the camera path in the plane of the view, with Ctrl along the line of sight")
                    onMoved: (to, done) => {
                        if (done)
                            paths.moveKey(one.modelData.id, modelData, false, to, held);
                    }
                }
            }

            SceneLightLines {
                anchors.fill: parent
                ratio: paths.stage.ratio
                dashed: true
                tint: Kirigami.Theme.positiveTextColor
                segments: one.modelData.keys.filter(k => k.at && k.aim).map(k => [k.at, k.aim])
            }

            SceneRepeater {
                id: listed

                rows: one.modelData.keys.filter(k => k.aim)

                delegate: SceneKnob {
                    required property int index
                    readonly property var modelData: listed.rows[index]

                    at: modelData.aim
                    ratio: paths.stage.ratio
                    square: true
                    label: App.tr("Aim at %1 s", modelData.t)
                    tint: Kirigami.Theme.positiveTextColor
                    tip: App.tr("Drag to move the point this key of the camera path looks at, with Ctrl along the line of sight")
                    onMoved: (to, done) => {
                        if (done)
                            paths.moveKey(one.modelData.id, modelData, true, to, held);
                    }
                }
            }
        }
    }

    SceneKnob {
        at: paths.info?.playing?.at ?? null
        ratio: paths.stage.ratio
        square: true
        label: App.tr("Camera now")
        tint: Kirigami.Theme.neutralTextColor
        tip: App.tr("Where the paths put the camera at this time")
    }
}
