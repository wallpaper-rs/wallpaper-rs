import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: groups

    required property var page
    required property var node
    readonly property var everyday: ["transform", "size", "opacity", "visible", "anchor"]
    readonly property var sections: [["blend", App.tr("Color and blending"), ["blend", "brightness", "lighting", "solid", "cache"]], ["depth", App.tr("Depth"), ["parallax", "depth_test", "attach", "inherit", "parent"]], ["mask", App.tr("Mask"), ["mask"]]]
    readonly property var kinds: ({
            "image": App.tr("Image and crop"),
            "text": App.tr("Lettering and color"),
            "shape": App.tr("Shape and fill"),
            "particles": App.tr("Particles"),
            "light": App.tr("Light"),
            "model": App.tr("Model and materials"),
            "mesh": App.tr("Mesh"),
            "rig": App.tr("Rig and bones"),
            "touch": App.tr("Touch and sensors")
        })
    readonly property var placed: [].concat(everyday, ["effects", "id", "type"], sections.reduce((all, section) => all.concat(section[2]), []))

    function head(field) {
        if (!node.pointer)
            return "";
        return field.pointer.slice(node.pointer.length + 1).split("/")[0];
    }

    function rows(keys) {
        return page.fields.filter(field => keys.includes(head(field)));
    }

    spacing: Look.e1

    SceneFieldList {
        Layout.fillWidth: true
        page: groups.page
        rows: groups.rows(groups.everyday)
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        Accessible.name: App.tr("Moves on its own")
        enabled: !groups.node.locked
        displayText: App.tr("Moves on its own: %1", App.tr("No"))
        model: starts.moves.map(move => move[2])
        onActivated: index => starts.keepMoving(starts.moves[index])

        SceneLayerStarts {
            id: starts

            owner: ({
                    "page": groups.page,
                    "current": groups.node
                })
        }
    }

    SceneEffectsCard {
        Layout.fillWidth: true
        visible: groups.node.type !== "sound"
        page: groups.page
        node: groups.node
        rows: groups.rows(["effects"])
    }

    SceneFold {
        id: kindFold

        readonly property var shown: groups.node.pointer ? groups.page.fields.filter(field => !groups.placed.includes(groups.head(field))) : []

        Layout.fillWidth: true
        visible: shown.length > 0
        key: "layer-kind"
        title: groups.kinds[groups.node.type] ?? App.tr("Content")
        open: false

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: kindFold.shown
        }
    }

    SceneFold {
        Layout.fillWidth: true
        visible: groups.rows(groups.sections[0][2]).length > 0
        key: "layer-blend"
        title: groups.sections[0][1]
        open: false

        SceneLayerLook {
            Layout.fillWidth: true
            page: groups.page
            node: groups.node
        }

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: groups.rows(groups.sections[0][2]).filter(field => field.key !== "blend")
        }
    }

    SceneFold {
        Layout.fillWidth: true
        visible: groups.rows(groups.sections[1][2]).length > 0
        key: "layer-depth"
        title: groups.sections[1][1]
        open: false

        SceneLayerDepth {
            Layout.fillWidth: true
            page: groups.page
            node: groups.node
            pointer: groups.node.pointer ?? ""
        }

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: groups.rows(groups.sections[1][2])
        }
    }

    SceneFold {
        Layout.fillWidth: true
        visible: groups.rows(groups.sections[2][2]).length > 0
        key: "layer-mask"
        title: groups.sections[2][1]
        open: false

        SceneFieldList {
            Layout.fillWidth: true
            page: groups.page
            rows: groups.rows(groups.sections[2][2])
        }
    }
}
