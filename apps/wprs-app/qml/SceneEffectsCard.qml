import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

SceneFold {
    id: card

    required property var page
    required property var node
    property var rows: []
    property int chosen: -1
    readonly property var list: {
        const found = rows.find(field => field.key === "effects" && field.depth === 0);
        return found && Array.isArray(found.shown) ? found.shown : [];
    }
    readonly property string at: node.pointer + "/effects/" + chosen
    readonly property var fields: {
        card.page.fields;
        return chosen >= 0 && chosen < list.length ? JSON.parse(page.editor.inspect(at) || "[]") : [];
    }

    key: "layer-effects"
    title: list.length > 0 ? App.tr("Effects · %1", list.length) : App.tr("Effects")
    onNodeChanged: chosen = -1

    Repeater {
        model: card.list

        delegate: QQC2.ToolButton {
            required property int index
            required property var modelData

            Layout.fillWidth: true
            icon.name: "sparkle"
            text: SceneWords.effect(modelData.effect ?? "")
            checkable: true
            checked: card.chosen === index
            onClicked: card.chosen = card.chosen === index ? -1 : index
        }
    }

    ColumnLayout {
        Layout.fillWidth: true
        Layout.leftMargin: Look.e3
        visible: card.fields.length > 0
        spacing: Look.e1

        SceneFieldList {
            Layout.fillWidth: true
            page: card.page
            rows: card.fields.filter(field => field.key !== "effect")
        }

        QQC2.ToolButton {
            icon.name: "arrow-square-out"
            text: App.tr("Open the effect")
            onClicked: card.page.editor.select(card.at)
        }
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        icon.name: "plus"
        text: App.tr("Add an effect")
        onClicked: catalog.open()
    }

    SceneEffectCatalog {
        id: catalog

        inspector: ({
                "page": card.page,
                "addEffect": id => card.page.run("effects", {
                            "add": {
                                "layer": card.node.id,
                                "effect": id
                            }
                        })
            })
    }
}
