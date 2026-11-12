import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: shade

    required property var row
    readonly property string shader: row.set.shader ?? "pbr"
    readonly property var params: row.set.params ?? {}
    readonly property var page: row.panel.page
    readonly property int layers: params.layers ?? 12

    function param(key, value) {
        const p = {};
        p[key] = Math.round(value * 1000) / 1000;
        row.panel.set(row.material.name, {
                "params": p
            });
    }

    visible: ["fur", "foliage", "chroma"].includes(shader) || (row.panel.info?.packages ?? []).includes(shader)

    SceneSliders {
        Layout.fillWidth: true
        visible: shade.shader === "fur"
        value: Object.assign({
                "layers": 12,
                "length": 10,
                "density": 0.5
            }, shade.params)
        rows: [["layers", App.tr("Shells"), 1, 32, 1], ["length", App.tr("Length, in hundredths of the model"), 0, 100, 1], ["density", App.tr("Density"), 0, 1, 0.01]]
        onEdited: (key, value) => shade.param(key, value)
    }

    QQC2.Label {
        visible: shade.shader === "fur"
        font: Kirigami.Theme.smallFont
        text: App.tr("Shells per quality level: %1, %2, %3 and %4", Math.max(1, Math.ceil(shade.layers / 4)), Math.ceil(shade.layers / 2), Math.ceil(shade.layers * 3 / 4), shade.layers)
    }

    SceneCombPad {
        visible: shade.shader === "fur"
        comb: [shade.params.comb_x ?? 0, shade.params.comb_y ?? 0, shade.params.comb_z ?? 0]
        onCombed: (axis, value) => shade.param(axis, value)
    }

    SceneMaterialBrush {
        Layout.fillWidth: true
        visible: shade.shader === "fur"
        row: shade.row
        shader: shade.shader
    }

    SceneSliders {
        Layout.fillWidth: true
        visible: shade.shader === "foliage"
        value: Object.assign({
                "strength": 1,
                "speed": 1
            }, shade.params)
        rows: [["strength", App.tr("Sway"), 0, 10, 0.1], ["speed", App.tr("Speed"), 0, 10, 0.1]]
        onEdited: (key, value) => shade.param(key, value)
    }

    SceneRigWind {
        Layout.fillWidth: true
        visible: shade.shader === "foliage"
        host: ({
                "page": shade.page
            })
    }

    RowLayout {
        visible: shade.shader === "chroma"

        QQC2.Label {
            text: App.tr("Key color")
        }

        SceneColor {
            alpha: false
            hex: typeof shade.params.key === "string" ? shade.params.key : "#00ff00"
            onPicked: hex => shade.row.panel.set(shade.row.material.name, {
                        "key": hex
                    })
        }
    }

    SceneMaterialBrush {
        Layout.fillWidth: true
        visible: shade.shader === "chroma"
        row: shade.row
        shader: shade.shader
    }

    SceneSliders {
        Layout.fillWidth: true
        visible: shade.shader === "chroma"
        value: Object.assign({
                "threshold": 0.4,
                "softness": 0.1
            }, shade.params)
        rows: [["threshold", App.tr("Tolerance"), 0, 1, 0.01], ["softness", App.tr("Feather"), 0, 1, 0.01]]
        onEdited: (key, value) => shade.param(key, value)
    }

    SceneMaterialPackage {
        Layout.fillWidth: true
        visible: (shade.row.panel.info?.packages ?? []).includes(shade.shader)
        shade: shade
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: (shade.row.panel.info?.packages ?? []).includes(shade.shader)
        type: Kirigami.MessageType.Warning
        text: App.tr("This shader comes with the package and runs on the graphics card within the limits of package effects; use only shaders you trust.")
    }
}
