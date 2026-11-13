import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: probe

    required property var page
    required property real ratio
    readonly property var held: page.view.probe ?? null
    readonly property var size: page.view.canvas?.size ?? [1920, 1080]
    readonly property bool gyro: page.view.canvas?.parallax?.gyro ?? false

    function send(args) {
        page.run("canvas", {
            "probe": args
        }, true);
    }

    function clamp(v, low, high) {
        return Math.max(low, Math.min(high, v));
    }

    visible: held !== null

    SceneOutline {
        anchors.fill: parent
        visible: probe.gyro
        corners: probe.held ? [[probe.size[0] / 2, probe.size[1] / 2], [probe.size[0] * (1 + probe.held.tilt[0]) / 2, probe.size[1] * (1 + probe.held.tilt[1]) / 2]] : []
        ratio: probe.ratio
        dashed: true
    }

    SceneKnob {
        at: probe.held ? [probe.held.pointer[0] * probe.size[0], probe.held.pointer[1] * probe.size[1]] : null
        ratio: probe.ratio
        tint: Look.brote
        label: App.tr("Pointer")
        tip: App.tr("Simulated pointer: drag it and every layer moves by its parallax depth")
        onMoved: to => probe.send({
                "pointer": [probe.clamp(to[0] / probe.size[0], 0, 1), probe.clamp(to[1] / probe.size[1], 0, 1)]
            })
    }

    SceneKnob {
        visible: probe.gyro && probe.held !== null
        at: probe.held ? [probe.size[0] * (1 + probe.held.tilt[0]) / 2, probe.size[1] * (1 + probe.held.tilt[1]) / 2] : null
        ratio: probe.ratio
        square: true
        tint: Look.brote
        label: App.tr("Tilt")
        tip: App.tr("Simulated tilt of a phone: drag it away from the middle to lean the phone that way")
        onMoved: to => probe.send({
                "tilt": [probe.clamp(to[0] / probe.size[0] * 2 - 1, -1, 1), probe.clamp(to[1] / probe.size[1] * 2 - 1, -1, 1)]
            })
    }
}
