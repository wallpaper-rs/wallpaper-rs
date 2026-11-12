import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: own

    required property var shade
    readonly property var page: shade.page
    readonly property var source: JSON.parse(page.editor.rig("", JSON.stringify({
                    "shader_source": {
                        "effect": shade.shader
                    }
                })) || "null") ?? {}
    readonly property var params: (source.parsed?.params ?? []).filter(p => ["float", "int", "color", "bool", "texture"].includes(p.type))
    readonly property var pictures: ["white", "black", "noise", "normal_flat"].concat((page.view.assets ?? []).filter(a => a.kind === "image").map(a => a.id))

    function set(id, value) {
        const values = {};
        values[id] = value;
        shade.row.panel.set(shade.row.material.name, {
                "values": values
            });
    }

    function current(p) {
        return shade.params[p.id] ?? p.default;
    }

    SceneRepeater {
        id: listed

        rows: own.params

        delegate: RowLayout {
            id: line

            required property int index
            readonly property var param: listed.rows[index]

            Layout.fillWidth: true

            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 6
                text: line.param.id
                elide: Text.ElideRight
            }

            QQC2.Slider {
                Layout.fillWidth: true
                visible: line.param.type === "float" || line.param.type === "int"
                from: line.param.min ?? 0
                to: line.param.max ?? 1
                stepSize: line.param.type === "int" ? 1 : 0
                value: Number(own.current(line.param)) || 0
                onMoved: own.set(line.param.id, line.param.type === "int" ? Math.round(value) : Math.round(value * 1000) / 1000)
            }

            SceneColor {
                visible: line.param.type === "color"
                hex: typeof own.current(line.param) === "string" ? own.current(line.param) : "#ffffff"
                onPicked: hex => own.set(line.param.id, hex)
            }

            QQC2.CheckBox {
                visible: line.param.type === "bool"
                checked: own.current(line.param) === true
                onToggled: own.set(line.param.id, checked)
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                visible: line.param.type === "texture"
                model: own.pictures
                currentIndex: Math.max(0, own.pictures.indexOf(String(own.current(line.param))))
                onActivated: index => own.set(line.param.id, own.pictures[index])
            }
        }
    }
}
