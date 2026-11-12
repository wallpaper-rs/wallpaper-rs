import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: overlays

    required property var page
    required property real ratio

    function nudge(step, repeated) {
        const chosen = (page.view.gizmos ?? []).find(g => g.pointer === page.editor.selected);
        if (!chosen || chosen.locked)
            return false;
        page.run("layer", {
                "grip": {
                    "layer": chosen.id,
                    "grip": "nudge",
                    "to": step,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            }, repeated);
        return true;
    }

    Connections {
        function onCanvasWanted() {
            overlays.forceActiveFocus();
        }

        target: SceneFocus
    }

    Keys.onPressed: event => {
        const by = (event.modifiers & Qt.ShiftModifier) ? 10 : 1;
        const steps = {
            [Qt.Key_Left]: [-by, 0],
            [Qt.Key_Right]: [by, 0],
            [Qt.Key_Up]: [0, -by],
            [Qt.Key_Down]: [0, by]
        };
        if (event.key === Qt.Key_Escape && page.picking !== "") {
            page.picking = "";
            event.accepted = true;
        } else if (steps[event.key]) {
            event.accepted = nudge(steps[event.key], event.isAutoRepeat);
        }
    }

    SceneWind {
        anchors.fill: parent
        z: 4
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneSwarm {
        anchors.fill: parent
        z: 5
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneLights {
        anchors.fill: parent
        z: 5
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneRgbRegion {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneMaskOverlay {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneEffectPointOverlay {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneTouchOverlay {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneShapeOverlay {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneTextOverlay {
        anchors.fill: parent
        z: 6
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneStage3d {
        anchors.fill: parent
        z: 5
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneCameraFrame {
        anchors.fill: parent
        z: 7
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneProbe {
        anchors.fill: parent
        z: 8
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneSnapGuides {
        anchors.fill: parent
        z: 7
        page: overlays.page
        ratio: overlays.ratio
    }

    SceneParentLink {
        anchors.fill: parent
        z: 7
        page: overlays.page
        ratio: overlays.ratio
    }

    Repeater {
        model: (overlays.page.view.gizmos ?? []).length

        delegate: SceneGizmo {
            page: overlays.page
            ratio: overlays.ratio
        }
    }
}
