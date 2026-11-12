import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: credits

    required property var host
    property var chosen: []
    property bool every: false
    readonly property var page: host.page
    readonly property var info: {
        page.view;
        return JSON.parse(page.editor.rig("", JSON.stringify({
                        "credits": {}
                    })) || "{}");
    }
    readonly property var assets: info.assets ?? []
    readonly property var shown: assets.filter(a => every || !a.credited).slice(0, 200)
    readonly property var held: info.credits ?? []

    function choose(path, on) {
        chosen = on ? chosen.concat([path]) : chosen.filter(p => p !== path);
    }

    function give() {
        page.run("manifest", {
                "credit": {
                    "paths": chosen,
                    "license": license.editText.trim(),
                    "attribution": who.text.trim(),
                    "url": link.text.trim() === "" ? null : link.text.trim()
                }
            });
        if (page.editor.error === "")
            chosen = [];
    }

    spacing: Kirigami.Units.smallSpacing

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: (info.missing ?? []).length > 0 ? App.tr("%1 files have no credit. Choose the ones someone else made and say who and under which license.", (info.missing ?? []).length) : App.tr("Every picture, sound, font and model of the package has a credit.")
    }

    QQC2.CheckBox {
        text: App.tr("Show the files that already have a credit")
        checked: credits.every
        onToggled: credits.every = checked
    }

    Repeater {
        model: credits.shown

        delegate: QQC2.CheckBox {
            required property var modelData

            Layout.fillWidth: true
            text: modelData.credited ? App.tr("%1 (credited)", modelData.path) : modelData.path
            checked: credits.chosen.includes(modelData.path)
            onToggled: credits.choose(modelData.path, checked)
        }
    }

    QQC2.ComboBox {
        id: license

        Layout.fillWidth: true
        editable: true
        model: ["CC0-1.0", "CC-BY-4.0", "CC-BY-SA-4.0", "MIT", "Apache-2.0", "GPL-3.0-or-later"]
        editText: ""
        Accessible.name: App.tr("License of the chosen files")
    }

    QQC2.TextField {
        id: who

        Layout.fillWidth: true
        placeholderText: App.tr("Who made them")
        Accessible.name: placeholderText
    }

    QQC2.TextField {
        id: link

        Layout.fillWidth: true
        placeholderText: App.tr("Address of the work (https://…)")
        Accessible.name: placeholderText
    }

    QQC2.Button {
        text: App.tr("Credit the chosen files")
        icon.name: "list-add"
        enabled: credits.chosen.length > 0 && license.editText.trim() !== "" && who.text.trim() !== ""
        onClicked: credits.give()
    }

    Repeater {
        model: credits.held

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: row.modelData.paths.join(", ") + ": " + row.modelData.attribution + " (" + row.modelData.license + ")"
                textFormat: Text.PlainText
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove this credit")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: credits.page.run("manifest", {
                        "uncredit": {
                            "paths": row.modelData.paths
                        }
                    })
            }
        }
    }
}
