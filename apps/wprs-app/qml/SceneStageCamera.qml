import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: cam

    required property var stage
    readonly property var handles: stage.info?.handles ?? null
    readonly property var rig: handles?.rig ?? null

    function move(what, from, to, held) {
        stage.grab(what, held & Qt.ControlModifier ? "depth" : "plane", from, to);
    }

    visible: handles !== null && stage.info?.through === false

    SceneKnob {
        at: cam.handles?.eye ?? null
        ratio: cam.stage.ratio
        label: App.tr("Camera")
        tint: Kirigami.Theme.neutralTextColor
        tip: App.tr("Drag to move the scene camera in the plane of the view, with Ctrl along the line of sight")
        onMoved: (to, done) => {
            if (done)
                cam.move("eye", cam.handles.eye, to, held);
        }
    }

    SceneKnob {
        at: cam.handles?.center ?? null
        ratio: cam.stage.ratio
        square: true
        label: App.tr("Aim")
        tint: Kirigami.Theme.neutralTextColor
        tip: App.tr("Drag to move what the scene camera looks at in the plane of the view, with Ctrl along the line of sight")
        onMoved: (to, done) => {
            if (done)
                cam.move("center", cam.handles.center, to, held);
        }
    }

    SceneLightLines {
        anchors.fill: parent
        ratio: cam.stage.ratio
        tint: Kirigami.Theme.positiveTextColor
        segments: cam.handles?.eye && cam.handles?.up ? [[cam.handles.eye, cam.handles.up]] : []
    }

    SceneStageRig {
        anchors.fill: parent
        cam: cam
    }
}
