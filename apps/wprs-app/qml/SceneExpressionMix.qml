import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: mix

    required property var host
    readonly property var mesh: host.page.mesh
    readonly property var expressions: host.info.expressions ?? {}
    property var levels: ({})

    function level(name, value) {
        const next = Object.assign({}, levels);
        next[name] = Math.round(value * 100) / 100;
        levels = next;
        const shapes = {};
        Object.keys(expressions).forEach(e => {
            Object.keys(expressions[e]).forEach(s => shapes[s] = (shapes[s] ?? 0) + expressions[e][s] * (levels[e] ?? 0));
        });
        Object.keys(shapes).forEach(s => shapes[s] = Math.max(-1, Math.min(1, Math.round(shapes[s] * 100) / 100)));
        mesh.layer = host.layerId;
        mesh.shapes = shapes;
    }

    visible: Object.keys(expressions).length > 0

    Connections {
        function onShapesChanged() {
            if (Object.keys(mix.mesh.shapes).length === 0)
                mix.levels = ({});
        }

        target: mix.mesh
    }

    QQC2.Label {
        text: App.tr("Expressions in the preview")
        font.bold: true
    }

    SceneRepeater {
        id: listed

        rows: Object.keys(mix.expressions)

        delegate: RowLayout {
            required property int index
            readonly property var modelData: listed.rows[index]

            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 5
                text: modelData
                elide: Text.ElideRight
            }

            SceneSlider {
                Layout.fillWidth: true
                from: 0
                to: 1
                value: mix.levels[modelData] ?? 0
                onMoved: mix.level(modelData, value)
            }
        }
    }
}
