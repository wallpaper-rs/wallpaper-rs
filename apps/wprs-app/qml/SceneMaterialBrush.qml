import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tools

    required property var row
    required property string shader
    readonly property var page: row.panel.page
    readonly property var tool: page.brush
    readonly property bool mine: tool?.layer === row.panel.figure && tool?.name === row.material.name
    readonly property bool combing: mine && tool?.action === "comb3d"
    readonly property bool keying: mine && tool?.action === "key3d"
    readonly property bool painted: typeof row.set.maps?.comb === "string"
    property real radius: 12
    property real strength: 1
    property bool erase: false

    function comb(on) {
        page.brush = on ? {
            "action": "comb3d",
            "layer": row.panel.figure,
            "name": row.material.name,
            "radius": radius,
            "strength": strength,
            "hardness": 0.5,
            "erase": erase
        } : null;
    }

    onRadiusChanged: combing && comb(true)
    onStrengthChanged: combing && comb(true)
    onEraseChanged: combing && comb(true)

    RowLayout {
        visible: tools.shader === "fur"

        QQC2.Button {
            checkable: true
            checked: tools.combing
            icon.name: "draw-brush"
            text: App.tr("Comb on the model")
            onToggled: tools.comb(checked)
        }

        QQC2.CheckBox {
            text: App.tr("Erase")
            checked: tools.erase
            onToggled: tools.erase = checked
        }

        QQC2.Button {
            visible: tools.painted
            icon.name: "edit-clear"
            text: App.tr("Clear the combing")
            onClicked: tools.row.panel.set(tools.row.material.name, {
                    "maps": {
                        "comb": null
                    }
                })
        }
    }

    SceneSliders {
        Layout.fillWidth: true
        visible: tools.shader === "fur" && tools.combing
        value: ({
                "radius": tools.radius,
                "strength": tools.strength
            })
        rows: [["radius", App.tr("Brush size, in pixels"), 2, 80, 1], ["strength", App.tr("Combing strength"), 0, 1, 0.05]]
        onEdited: (key, value) => tools[key] = value
    }

    QQC2.Button {
        visible: tools.shader === "chroma"
        checkable: true
        checked: tools.keying
        icon.name: "color-picker"
        text: App.tr("Pick the key color from the model")
        onToggled: tools.page.brush = checked ? {
            "action": "key3d",
            "layer": tools.row.panel.figure,
            "name": tools.row.material.name
        } : null
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: tools.combing || tools.keying
        text: tools.combing ? App.tr("Drag over the model in the 3D view to comb the fur where the pointer passes; the combing is painted into a map of the material.") : App.tr("Click the model in the 3D view to take its color as the key.")
    }
}
