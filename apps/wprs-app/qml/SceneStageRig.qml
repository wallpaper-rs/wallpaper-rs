import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: rig

    required property var cam
    readonly property var parts: cam.rig

    SceneLightLines {
        anchors.fill: parent
        ratio: rig.cam.stage.ratio
        tint: Look.brote
        segments: rig.parts?.roll ?? []
    }

    SceneKnob {
        at: rig.parts?.roll_grip ?? null
        ratio: rig.cam.stage.ratio
        label: App.tr("Roll")
        tint: Look.brote
        tip: App.tr("Drag along the ring to roll the scene camera around its line of sight")
        onMoved: (to, done) => {
            if (done)
                rig.cam.stage.grab("eye", "roll", rig.parts.roll_grip, to);
        }
    }

    SceneKnob {
        at: rig.parts?.orbit ?? null
        ratio: rig.cam.stage.ratio
        label: App.tr("Orbit")
        tint: Look.ambar
        tip: App.tr("Drag to swing the scene camera around what it looks at")
        onMoved: (to, done) => {
            if (done)
                rig.cam.stage.grab("eye", "orbit", rig.parts.orbit, to);
        }
    }

    SceneKnob {
        at: rig.parts?.dolly ?? null
        ratio: rig.cam.stage.ratio
        square: true
        label: App.tr("Dolly")
        tint: Look.ambar
        tip: App.tr("Drag up to bring the scene camera closer to what it looks at, down to back away")
        onMoved: (to, done) => {
            if (done)
                rig.cam.stage.grab("eye", "dolly", rig.parts.dolly, to);
        }
    }
}
