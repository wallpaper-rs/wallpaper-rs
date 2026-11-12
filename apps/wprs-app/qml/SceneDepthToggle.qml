import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: toggle

    required property var page
    required property var node
    readonly property bool group: node?.type === "group"
    readonly property bool deep: group && JSON.parse(page.editor.value(node.pointer + "/space") || "null") === "3d"
    readonly property bool inside: {
        const up = String(node?.pointer ?? "").replace(/\/layers\/\d+$/, "");
        return up !== String(node?.pointer ?? "") && JSON.parse(page.editor.value(up + "/space") || "null") === "3d";
    }

    property var way: null
    readonly property var reasons: ({
            "perspective": App.tr("its particle system has no perspective"),
            "lighting": App.tr("the layer takes light"),
            "lit": App.tr("its particles are lit"),
            "refract": App.tr("its particles bend what lies behind them"),
            "shader": App.tr("its particles use their own shader"),
            "trail": App.tr("its particles draw trails"),
            "rope": App.tr("its particles draw ropes"),
            "children": App.tr("a child system is drawn that way or has children of its own")
        })

    function load() {
        if (node?.type !== "particles") {
            way = null;
            return;
        }
        way = JSON.parse(page.editor.rig("", JSON.stringify({
            "depth3d": {
                "layer": node.id
            }
        })) || "null");
    }

    visible: node !== null && (group || (inside && !["model", "light", "sound"].includes(node.type)))
    onNodeChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            toggle.load();
        }

        target: toggle.page.editor
    }

    QQC2.CheckBox {
        visible: toggle.group
        text: App.tr("3D group: models, lights and flat layers share depth")
        checked: toggle.deep
        onToggled: toggle.page.run("scene3d", {
            "space": {
                "layer": toggle.node.id,
                "deep": checked
            }
        })
    }

    QQC2.CheckBox {
        visible: !toggle.group
        text: App.tr("Hide behind nearer models")
        checked: toggle.node !== null && toggle.page.editor.value(toggle.node.pointer + "/depth_test") === "true"
        onToggled: toggle.page.run("scene3d", {
            "depth_test": {
                "layer": toggle.node.id,
                "on": checked
            }
        })
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: !toggle.group && toggle.way !== null && toggle.way.particles && toggle.way.depth_test
        wrapMode: Text.WordWrap
        opacity: 0.8
        text: toggle.way?.way === "one_by_one" ? App.tr("Each particle hides on its own behind nearer models; at the lowest quality the whole layer hides as one plane.") : App.tr("The whole layer hides as one plane, because %1.").arg(toggle.reasons[toggle.way?.reason] ?? "")
    }
}
