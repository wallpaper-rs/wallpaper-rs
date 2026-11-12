import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: items

    required property var input
    readonly property var rows: Array.isArray(input.value) ? input.value : []
    readonly property var owner: input.field.key === "effects" ? (input.page.view.layers ?? []).find(layer => layer.pointer === input.field.pointer.replace(/\/effects$/, "")) : undefined

    QQC2.Label {
        Layout.fillWidth: true
        text: items.rows.length === 1 ? App.tr("1 item") : App.tr("%1 items", items.rows.length)
    }

    QQC2.Button {
        icon.name: "list-add"
        text: App.tr("Add")
        onClicked: items.owner ? picker.open() : input.page.run(input.field.set ? "add" : "set", {
                "pointer": input.field.pointer + (input.field.set ? "/-" : ""),
                "value": input.field.set ? input.field.sample : [input.field.sample]
            })
    }

    SceneEffectCatalog {
        id: picker

        inspector: ({
                "page": input.page,
                "addEffect": id => input.page.run("effects", {
                            "add": {
                                "layer": items.owner.id,
                                "effect": id
                            }
                        })
            })
    }

    QQC2.ComboBox {
        model: items.rows.map((_, i) => App.tr("Item %1", i + 1))
        displayText: App.tr("Edit…")
        enabled: count > 0
        onActivated: index => input.page.editor.select(input.field.pointer + "/" + index)
    }
}
