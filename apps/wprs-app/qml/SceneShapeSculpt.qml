import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sculpt

    required property var host
    readonly property var mesh: host.page.mesh
    readonly property bool carving: mesh.carving !== null && mesh.rigging === host.layerId
    readonly property var names: host.info.shapes ?? []
    readonly property var modes: ["push", "pull", "smooth", "erase"]
    readonly property string shape: pick.currentIndex < names.length ? names[pick.currentIndex] : fresh.text.trim()
    readonly property bool pushing: mode.currentIndex < 2

    function brush() {
        return {
            "shape": shape,
            "mode": modes[mode.currentIndex],
            "by": Math.round(strength.value * (pushing ? 50 : 1) * 100) / 100,
            "radius": Math.round(size.value * 1000) / 1000,
            "falloff": Math.round(soft.value * 100) / 100
        };
    }

    function update() {
        if (carving)
            mesh.carving = brush();
        show();
    }

    function show() {
        if (!carving || !names.includes(shape))
            return;
        mesh.layer = host.layerId;
        const only = {};
        only[shape] = 1;
        mesh.shapes = only;
    }

    visible: host.editable && !host.solid
    onNamesChanged: show()

    QQC2.Button {
        text: App.tr("Sculpt a blend shape on the preview")
        icon.name: "draw-brush"
        checkable: true
        checked: sculpt.carving
        enabled: sculpt.shape !== "" || checked
        onToggled: {
            sculpt.mesh.tool(sculpt.host.layerId, "shape", checked ? sculpt.brush() : null);
            sculpt.show();
        }
    }

    GridLayout {
        columns: 2
        visible: sculpt.carving

        QQC2.Label {
            text: App.tr("Shape")
        }

        QQC2.ComboBox {
            id: pick

            Layout.fillWidth: true
            model: sculpt.names.concat([App.tr("New shape…")])
            onActivated: sculpt.update()
        }

        QQC2.Label {
            visible: fresh.visible
            text: App.tr("Name")
        }

        QQC2.TextField {
            id: fresh

            Layout.fillWidth: true
            visible: pick.currentIndex >= sculpt.names.length
            placeholderText: App.tr("Name of the new blend shape")
            onTextEdited: sculpt.update()
        }

        QQC2.Label {
            text: App.tr("Brush")
        }

        QQC2.ComboBox {
            id: mode

            Layout.fillWidth: true
            model: [App.tr("Push away"), App.tr("Pull in"), App.tr("Smooth"), App.tr("Erase")]
            onActivated: sculpt.update()
        }

        QQC2.Label {
            text: App.tr("Strength")
        }

        SceneSlider {
            id: strength

            Layout.fillWidth: true
            from: 0.02
            to: 1
            value: 0.3
            onMoved: sculpt.update()
        }

        QQC2.Label {
            text: App.tr("Brush size")
        }

        SceneSlider {
            id: size

            Layout.fillWidth: true
            from: 0.01
            to: 0.5
            value: 0.1
            onMoved: sculpt.update()
        }

        QQC2.Label {
            text: App.tr("Soft edge")
        }

        SceneSlider {
            id: soft

            Layout.fillWidth: true
            from: 0
            to: 1
            value: 0.5
            onMoved: sculpt.update()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: sculpt.carving
        wrapMode: Text.Wrap
        text: App.tr("Strokes on the mesh move its vertices in the blend shape; the preview shows the shape at full weight. A new name makes the shape on the first stroke.")
    }
}
