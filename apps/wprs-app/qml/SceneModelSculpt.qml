import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sculpt

    required property var motion
    readonly property var page: motion.page
    readonly property bool painting: page.mesh.carving !== null && page.mesh.rigging === motion.layerId
    readonly property var modes: ["push", "pull", "smooth"]
    property int mode: 0

    function brush() {
        if (painting)
            page.mesh.tool(motion.layerId, "carve", settings());
    }

    function settings() {
        return {
            "shape": name.text.trim(),
            "mode": modes[mode],
            "radius": Math.round(size.value),
            "strength": Math.round((mode === 2 ? smooth.value : amount.value) * 10000) / 10000
        };
    }

    visible: motion.info?.model ?? false

    QQC2.Label {
        text: App.tr("Sculpt a blend shape")
        font.bold: true
    }

    RowLayout {
        QQC2.TextField {
            id: name

            Layout.fillWidth: true
            text: "sculpted_1"
            placeholderText: App.tr("Name of the shape")
            onTextEdited: sculpt.brush()
        }

        QQC2.Button {
            icon.name: "draw-brush"
            text: App.tr("Sculpt on the view")
            checkable: true
            checked: sculpt.painting
            enabled: /^[A-Za-z0-9_-]{1,64}$/.test(name.text.trim())
            onToggled: sculpt.page.mesh.tool(sculpt.motion.layerId, checked ? "carve" : "", checked ? sculpt.settings() : null)
        }
    }

    RowLayout {
        Repeater {
            model: [App.tr("Push"), App.tr("Pull"), App.tr("Smooth")]

            delegate: QQC2.Button {
                required property int index
                required property string modelData

                text: modelData
                checkable: true
                checked: sculpt.mode === index
                onClicked: {
                    sculpt.mode = index;
                    sculpt.brush();
                }
            }
        }
    }

    RowLayout {
        QQC2.Label {
            text: App.tr("Size")
        }

        SceneSlider {
            id: size

            Layout.fillWidth: true
            from: 4
            to: 300
            value: 40
            onMoved: sculpt.brush()
        }
    }

    RowLayout {
        QQC2.Label {
            text: App.tr("Strength")
        }

        SceneSlider {
            id: amount

            Layout.fillWidth: true
            visible: sculpt.mode !== 2
            from: 0.001
            to: 0.5
            value: 0.05
            onMoved: sculpt.brush()
        }

        SceneSlider {
            id: smooth

            Layout.fillWidth: true
            visible: sculpt.mode === 2
            from: 0.05
            to: 1
            value: 0.5
            onMoved: sculpt.brush()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("Each stroke moves the vertices under the brush along their normals, or evens them out, and saves the shape in a file beside the model; Undo takes the stroke back. The shape then has a weight slider like those of the file.")
    }
}
