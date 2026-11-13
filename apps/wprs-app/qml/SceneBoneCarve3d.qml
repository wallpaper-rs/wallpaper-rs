import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: carve

    required property var bones
    readonly property var page: bones.stage.page
    readonly property var brush: page.mesh.carving
    readonly property bool active: brush !== null && page.mesh.rigging === bones.model.id
    readonly property real ratio: bones.stage.ratio

    visible: active

    Rectangle {
        visible: hover.hovered || drag.active
        x: (drag.active ? drag.centroid.position.x : hover.point.position.x) - width / 2
        y: (drag.active ? drag.centroid.position.y : hover.point.position.y) - height / 2
        width: (carve.brush?.radius ?? 0) * 2 * carve.ratio
        height: width
        radius: width / 2
        color: "transparent"
        border.width: 2
        border.color: carve.brush?.mode === "pull" ? Look.coral : carve.brush?.mode === "smooth" ? Look.ambar : Look.brote
    }

    HoverHandler {
        id: hover

        enabled: carve.active
    }

    DragHandler {
        id: drag

        property var points: []

        target: null
        enabled: carve.active
        onActiveChanged: {
            if (!active && points.length > 0)
                carve.page.run("scene3d", {
                    "sculpt": Object.assign({}, carve.brush, {
                        "layer": carve.bones.model.id,
                        "points": points,
                        "time": carve.page.editor.time
                    })
                });
            points = [];
        }
        onCentroidChanged: {
            const p = centroid.position;
            if (active && points.length < 256)
                points = points.concat([[Math.round(p.x / carve.ratio * 10) / 10, Math.round(p.y / carve.ratio * 10) / 10]]);
        }
    }
}
