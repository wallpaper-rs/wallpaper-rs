import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: picker

    required property var host
    readonly property var field: host.field
    readonly property bool chain: field.key === "fallback"
    readonly property string base: field.pointer.replace(/\/(font|fallback)$/, "")
    readonly property var owner: (host.page.view.layers ?? []).find(l => l.pointer === base) ?? null
    readonly property var value: host.value
    readonly property var names: chain ? (Array.isArray(value) ? value : []) : (typeof value === "string" && value !== "" ? [value] : [])
    property var listing: ({})

    function state(name) {
        return (listing.chain ?? []).find(c => c.name === name)?.source ?? "";
    }

    function refresh() {
        if (owner)
            listing = JSON.parse(host.page.editor.rig("", JSON.stringify({
                            "font_list": {
                                "layer": owner.id
                            }
                        })) || "{}");
    }

    function apply(font, fallback) {
        host.page.run("layer", {
                "figure": {
                    "fonts": {
                        "layer": owner.id,
                        "font": font,
                        "fallback": fallback
                    }
                }
            });
    }

    function pick(name) {
        if (!chain)
            return apply(name, undefined);
        apply(undefined, names.concat([name]));
    }

    function shift(index, by) {
        const next = names.slice();
        next.splice(index + by, 0, next.splice(index, 1)[0]);
        apply(undefined, next);
    }

    Component.onCompleted: refresh()
    onValueChanged: refresh()

    SceneFontList {
        id: choices

        listing: picker.listing
        folder: picker.host.page.editor.folder
        onPicked: name => picker.pick(name)
    }

    RowLayout {
        visible: !picker.chain
        Layout.fillWidth: true

        QQC2.Button {
            Layout.fillWidth: true
            text: picker.names[0] ?? App.tr("Default font")
            font.family: picker.names[0] ?? Kirigami.Theme.defaultFont.family
            onClicked: choices.open()
        }

        QQC2.ToolButton {
            visible: picker.names.length > 0
            icon.name: "edit-clear"
            text: App.tr("Use the default font")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: picker.apply("", undefined)
        }
    }

    SceneRepeater {
        id: rows

        rows: picker.chain ? picker.names : []

        delegate: RowLayout {
            id: line

            required property int index
            readonly property string modelData: rows.rows[index]

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                text: line.modelData
                font.family: line.modelData
                elide: Text.ElideRight
            }

            QQC2.ToolButton {
                icon.name: "go-up"
                text: App.tr("Move up")
                display: QQC2.AbstractButton.IconOnly
                enabled: line.index > 0
                onClicked: picker.shift(line.index, -1)
            }

            QQC2.ToolButton {
                icon.name: "go-down"
                text: App.tr("Move down")
                display: QQC2.AbstractButton.IconOnly
                enabled: line.index < picker.names.length - 1
                onClicked: picker.shift(line.index, 1)
            }

            QQC2.ToolButton {
                icon.name: "list-remove"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                onClicked: picker.apply(undefined, picker.names.filter((_, k) => k !== line.index))
            }
        }
    }

    QQC2.Button {
        visible: picker.chain
        enabled: picker.names.length < (picker.listing.most ?? 8)
        icon.name: "list-add"
        text: App.tr("Add a fallback font")
        onClicked: choices.open()
    }

    SceneRepeater {
        id: missing

        rows: picker.names.filter(n => picker.state(n) === "missing")

        delegate: QQC2.Label {
            required property int index

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: Look.ambar
            text: App.tr("%1 is not in the package or installed here, so the next font is used", missing.rows[index])
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: picker.chain
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("A letter the font does not have is drawn with the first font of this list that has it")
    }
}
