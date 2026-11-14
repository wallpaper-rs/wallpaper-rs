import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: inspector

    required property var page
    readonly property string pointer: page.editor.selected
    readonly property var parts: pointer.split("/").slice(1)
    readonly property var node: (page.view.layers ?? []).find(item => item.pointer === pointer) ?? null
    readonly property string up: {
        const tokens = pointer.split("/");
        const last = tokens.pop();
        if (/^\d+$/.test(last) || ["particles", "properties", "assets", "params"].includes(tokens[tokens.length - 1]))
            tokens.pop();
        return tokens.length > 1 ? tokens.join("/") : "";
    }

    readonly property string list: node === null && /\/\d+$/.test(pointer) ? pointer.replace(/\/\d+$/, "") : ""
    readonly property int index: list ? Number(parts[parts.length - 1]) : -1
    readonly property int count: page.view && list ? page.editor.count(list) : 0

    function crumb(part, i) {
        const rows = i === 2 && parts[0] === "manifest" ? page.view[parts[1]] : undefined;
        const found = Array.isArray(rows) ? rows.find(row => row.pointer === "/" + parts.slice(0, 3).join("/")) : undefined;
        return found?.label ?? SceneWords.label(part, "/" + parts.slice(0, i + 1).join("/"));
    }

    function shift(by) {
        const to = list + "/" + (index + by);
        page.run("move", {
                "from": pointer,
                "to": to
            });
        page.editor.select(to);
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: inspector.parts.length > 1
        text: inspector.parts.slice(0, -1).map((part, i) => inspector.crumb(part, i)).join(" › ")
        color: Look.bruma
        font.pixelSize: Look.tPie
        elide: Text.ElideLeft
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: inspector.node ? inspector.node.id : inspector.crumb(inspector.parts[inspector.parts.length - 1] ?? "", inspector.parts.length - 1)
        font.pixelSize: Look.tNombre
        font.weight: Font.DemiBold
        elide: Text.ElideRight
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: inspector.node !== null
        text: inspector.node ? SceneWords.kind(inspector.node.type) : ""
        color: Look.bruma
        font.pixelSize: Look.tPie
    }

    RowLayout {
        Layout.fillWidth: true
        visible: inspector.up !== "" || inspector.list !== "" || (inspector.node !== null && inspector.node.type === "particles")

        QQC2.ToolButton {
            icon.name: "go-previous"
            text: App.tr("Up")
            display: QQC2.AbstractButton.IconOnly
            enabled: inspector.up !== ""
            onClicked: inspector.page.editor.select(inspector.up)
        }

        QQC2.ToolButton {
            visible: inspector.list !== ""
            icon.name: "go-up"
            text: App.tr("Move up")
            display: QQC2.AbstractButton.IconOnly
            enabled: inspector.index > 0
            onClicked: inspector.shift(-1)
        }

        QQC2.ToolButton {
            visible: inspector.list !== ""
            icon.name: "go-down"
            text: App.tr("Move down")
            display: QQC2.AbstractButton.IconOnly
            enabled: inspector.index < inspector.count - 1
            onClicked: inspector.shift(1)
        }

        QQC2.ToolButton {
            visible: inspector.node === null && /\/\d+$/.test(inspector.pointer)
            icon.name: "edit-delete"
            text: App.tr("Remove")
            display: QQC2.AbstractButton.IconOnly
            onClicked: {
                inspector.page.run("remove", {
                        "pointer": inspector.pointer
                    });
                inspector.page.editor.select(inspector.up);
            }
        }

        QQC2.ToolButton {
            visible: inspector.node?.type === "particles"
            icon.name: "weather-snow"
            text: App.tr("Edit the particle system")
            display: QQC2.AbstractButton.IconOnly
            onClicked: inspector.page.editor.select(SceneSwarmKnobs.pointer(String(inspector.page.fields.find(f => f.key === "system")?.value ?? ""), ""))
        }
    }

    QQC2.ScrollView {
        id: scroll

        Layout.fillWidth: true
        Layout.maximumWidth: inspector.width
        Layout.fillHeight: true
        contentWidth: availableWidth
        clip: true

        ColumnLayout {
            width: scroll.availableWidth

            SceneLayerFields {
                Layout.fillWidth: true
                visible: inspector.node !== null
                page: inspector.page
                node: inspector.node ?? {}
            }

            SceneShaping {
                Layout.fillWidth: true
                page: inspector.page
                node: inspector.node
                pointer: inspector.pointer
            }

            SceneLightTools {
                Layout.fillWidth: true
                page: inspector.page
                node: inspector.node
            }

            SceneLighting {
                Layout.fillWidth: true
                page: inspector.page
                node: inspector.node
            }

            SceneLayerLinks {
                Layout.fillWidth: true
                page: inspector.page
                node: inspector.node
            }

            SceneLightsPanel {
                Layout.fillWidth: true
                visible: inspector.pointer === "/scene/canvas"
                page: inspector.page
            }

            SceneSolids {
                Layout.fillWidth: true
                page: inspector.page
                node: inspector.node
                pointer: inspector.pointer
            }

            SceneSwarmTools {
                Layout.fillWidth: true
                page: inspector.page
                system: /^\/scene\/particles\/[^\/]+$/.test(inspector.pointer) ? inspector.parts[2].replace(/~1/g, "/").replace(/~0/g, "~") : ""
            }

            SceneEffectCost {
                Layout.fillWidth: true
                visible: /\/effects\/\d+$/.test(inspector.pointer)
                page: inspector.page
            }

            ScenePropLevels {
                Layout.fillWidth: true
                page: inspector.page
            }

            SceneFieldList {
                Layout.fillWidth: true
                visible: inspector.node === null
                page: inspector.page
                rows: inspector.node === null ? inspector.page.fields : []
            }
        }
    }
}
