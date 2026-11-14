import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var layers: page.view.layers ?? []
    readonly property var current: layers.find(layer => layer.pointer === page.editor.selected) ?? null
    readonly property var siblings: current ? layers.filter(layer => layer.pointer.replace(/\/\d+$/, "") === current.pointer.replace(/\/\d+$/, "")) : []
    readonly property var shown: {
        const out = [];
        const visit = parent => layers.filter(layer => layer.pointer.replace(/\/\d+$/, "") === parent).reverse().forEach(layer => {
                out.push(layer);
                visit(layer.pointer + "/layers");
            });
        visit("/scene/layers");
        return out;
    }
    readonly property int index: current ? Number(current.pointer.split("/").pop()) : -1
    readonly property string group: {
        const parent = current ? current.pointer.replace(/\/layers\/\d+$/, "") : "";
        return layers.find(layer => layer.pointer === parent)?.id ?? "";
    }

    property string target

    readonly property var groups: current ? layers.filter(layer => layer.type === "group" && layer.pointer !== current.pointer && !layer.pointer.startsWith(current.pointer + "/")) : []

    function pick(layer, add) {
        if (!add)
            page.picked = [];
        else if (page.picked.includes(layer.id))
            page.picked = page.picked.filter(id => id !== layer.id);
        else
            page.picked = page.picked.concat([layer.id]);
        page.editor.select(layer.pointer);
        SceneFocus.canvasWanted();
    }

    function move(to, into) {
        const args = {
            "id": current.id,
            "to": to
        };
        if (into)
            args.into = into;
        page.run("move_layer", args);
    }

    function drop(layer, onto) {
        target = "";
        if (!onto || onto.pointer === layer.pointer || onto.pointer.startsWith(layer.pointer + "/"))
            return;
        const parent = onto.pointer.replace(/\/layers\/\d+$/, "");
        const args = {
            "id": layer.id,
            "to": Number(onto.pointer.split("/").pop())
        };
        const group = layers.find(other => other.pointer === parent)?.id;
        if (group)
            args.into = group;
        page.run("move_layer", args);
    }

    function add() {
        tools.add();
    }

    function icon(type) {
        return ({
                "image": "image-x-generic",
                "video": "video-x-generic",
                "text": "draw-text",
                "shape": "draw-rectangle",
                "solid": "fill-color",
                "spectrum": "view-statistics",
                "particles": "weather-snow",
                "sound": "audio-x-generic",
                "group": "folder",
                "adjustment": "color-management",
                "light": "help-hint",
                "model": "draw-cuboid",
                "group3d": "folder-3d",
                "shapes": "draw-polyline"
            })[type] ?? "layer-visible-on";
    }

    SceneLayerTools {
        id: tools

        Layout.fillWidth: true
        Layout.margins: Look.e3
        owner: panel
    }

    QQC2.Label {
        Layout.fillWidth: true
        Layout.margins: Look.e3
        visible: (panel.page.view.layers ?? []).length === 0
        wrapMode: Text.WordWrap
        color: Look.brumaBaja
        font.pixelSize: Look.tPie
        text: App.tr("There are no layers yet. The first one you add stays at the back; the next ones go on top.")
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true

        ListView {
            model: panel.shown
            clip: true

            delegate: SceneLayerRow {
                owner: panel
            }
        }
    }
}
