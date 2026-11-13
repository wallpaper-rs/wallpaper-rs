import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: gizmo

    required property var owner
    required property var light
    readonly property real ratio: owner.ratio
    readonly property bool chosen: owner.selected === light.pointer
    readonly property var at: light.at

    function keyed(args) {
        owner.page.run("light", {
            "set": Object.assign({
                "layer": light.id,
                "time": owner.page.editor.time,
                "replace": owner.page.overwrite
            }, args)
        }, true);
    }

    function aim(to) {
        keyed({
            "aim": to.map(v => Math.round(v * 100) / 100)
        });
    }

    function tilt(to) {
        const aim = light.aim, dx = aim[0] - at[0], dy = aim[1] - at[1];
        const along = ((to[0] - at[0]) * dx + (to[1] - at[1]) * dy) / Math.max(dx * dx + dy * dy, 1e-6);
        keyed({
            "tilt": Math.max(-89, Math.min(89, Math.round(90 - 180 * (along - 0.1) / 0.8)))
        });
    }

    function cone(to, inner) {
        const turn = Math.atan2(to[1] - at[1], to[0] - at[0]) * 180 / Math.PI - light.angle;
        const wide = Math.min(180, Math.round(Math.abs(((turn + 540) % 360) - 180) * 20) / 10);
        keyed({
            "cone": inner ? [Math.min(wide, light.cone[1]), light.cone[1]] : [light.cone[0], Math.max(wide, light.cone[0])]
        });
    }

    function end(to, other, plus) {
        const mid = [(to[0] + other[0]) / 2, (to[1] + other[1]) / 2];
        keyed({
            "at": mid.map(v => Math.round(v * 100) / 100),
            "aim": (plus ? to : other).map(v => Math.round(v * 100) / 100),
            "length": Math.max(1, Math.round(Math.hypot(to[0] - other[0], to[1] - other[1])))
        });
    }

    Rectangle {
        visible: gizmo.chosen || gizmo.light.type === "point"
        x: (gizmo.at[0] - gizmo.light.radius) * gizmo.ratio
        y: (gizmo.at[1] - gizmo.light.radius) * gizmo.ratio
        width: gizmo.light.radius * 2 * gizmo.ratio
        height: width
        radius: width / 2
        color: "transparent"
        border.color: Look.ambar
        opacity: gizmo.chosen ? 0.9 : 0.35
    }

    Rectangle {
        visible: gizmo.chosen && gizmo.owner.page.lightBoxes === true
        x: gizmo.light.box[0] * gizmo.ratio
        y: gizmo.light.box[1] * gizmo.ratio
        width: gizmo.light.box[2] * gizmo.ratio
        height: gizmo.light.box[3] * gizmo.ratio
        color: "transparent"
        border.color: Look.coral
        border.width: 2
    }

    SceneLightCone {
        anchors.fill: parent
        gizmo: gizmo
    }

    SceneLightLines {
        anchors.fill: parent
        visible: gizmo.chosen
        ratio: gizmo.ratio
        segments: (gizmo.light.outer ?? []).concat(gizmo.light.inner ?? []).map(p => [gizmo.at, p]).concat(gizmo.light.aim ? [[gizmo.at, gizmo.light.aim]] : []).concat(gizmo.light.ends ? [gizmo.light.ends] : []).concat([[gizmo.at, gizmo.light.z_knob]])
    }

    SceneKnob {
        at: gizmo.at
        ratio: gizmo.ratio
        label: gizmo.light.id
        tint: gizmo.light.muted ? Kirigami.Theme.disabledTextColor : Look.ambar
        tip: App.tr("Drag to move the light")
        onMoved: (to, done) => {
            gizmo.owner.page.editor.select(gizmo.light.pointer);
            seat.grip("move", to, true);
        }
    }

    SceneSeat {
        id: seat

        page: gizmo.owner.page
        ratio: gizmo.ratio
        layerId: gizmo.light.id
    }

    SceneLightKnobs {
        gizmo: gizmo
    }
}
