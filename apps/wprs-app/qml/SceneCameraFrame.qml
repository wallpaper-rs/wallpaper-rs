import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: frame

    required property var page
    required property real ratio
    readonly property var camera: page.view.camera2d ?? null

    function grip(hold, to) {
        page.run("canvas", {
            "camera_grip": {
                "hold": hold,
                "to": to,
                "at": page.editor.time,
                "replace": page.overwrite
            }
        }, true);
    }

    visible: camera !== null && camera.whole

    SceneOutline {
        anchors.fill: parent
        corners: frame.camera?.corners ?? []
        ratio: frame.ratio
        chosen: true
    }

    SceneOutline {
        anchors.fill: parent
        corners: frame.camera ? [frame.camera.center, frame.camera.top] : []
        ratio: frame.ratio
        dashed: true
    }

    Repeater {
        model: [[0, 0], [1, 0], [1, 1], [0, 1]]

        delegate: SceneKnob {
            required property var modelData
            required property int index

            at: frame.camera?.corners[index] ?? null
            ratio: frame.ratio
            square: true
            tint: Look.ambar
            tip: App.tr("Camera zoom %1×: drag a corner of the frame closer to zoom in", frame.camera?.zoom ?? 1)
            onMoved: to => frame.grip({
                    "zoom": {
                        "corner": modelData
                    }
                }, to)
        }
    }

    SceneKnob {
        at: frame.camera?.center ?? null
        ratio: frame.ratio
        tint: Look.ambar
        label: App.tr("Camera")
        tip: App.tr("Center of the camera: drag to move what it shows")
        onMoved: to => frame.grip("move", to)
    }

    SceneKnob {
        at: frame.camera?.top ?? null
        ratio: frame.ratio
        tint: Look.ambar
        tip: App.tr("Camera turned %1°: drag to turn the top of the view toward the pointer", frame.camera?.rotation ?? 0)
        onMoved: to => frame.grip("turn", to)
    }
}
