import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: systems

    required property var page
    property string system: ""
    readonly property var ids: {
        page.view;
        return Object.keys(JSON.parse(page.editor.value("/scene/particles") || "null") ?? {});
    }
    property var renamed: null

    function fresh(base) {
        let n = 1;
        while (ids.includes(base + "_" + n))
            n++;
        return base + "_" + n;
    }

    function uses(name) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "script_uses": {
                        "name": name
                    }
                })) || "[]");
    }

    function open(name) {
        page.editor.select(SceneSwarmKnobs.pointer(name, ""));
    }

    function rename(from, to) {
        if (to === from || to === "")
            return;
        const found = uses(from);
        page.run("particles", {
                "rename": {
                    "system": from,
                    "to": to
                }
            });
        if (page.editor.error !== "")
            return;
        renamed = found.length ? {
                "from": from,
                "to": to,
                "uses": found
            } : null;
        if (from === system)
            open(to);
    }

    function create(args, select) {
        page.run("particles", {
                "create": args
            });
        if (select && page.editor.error === "")
            open(args.system);
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Particle systems of the scene")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: systems.ids.length === 0
        text: App.tr("The scene has no particle systems yet.")
    }

    SceneRepeater {
        id: listed

        rows: systems.ids

        delegate: RowLayout {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true

            QQC2.TextField {
                Layout.fillWidth: true
                text: modelData
                font.bold: modelData === systems.system
                validator: RegularExpressionValidator {
                    regularExpression: /[a-z0-9_]{1,32}/
                }
                onEditingFinished: systems.rename(modelData, text)
            }

            QQC2.ToolButton {
                icon.name: "document-edit"
                text: App.tr("Edit this system")
                display: QQC2.AbstractButton.IconOnly
                enabled: modelData !== systems.system
                onClicked: systems.open(modelData)
            }

            QQC2.ToolButton {
                icon.name: "edit-copy"
                text: App.tr("Copy this system")
                display: QQC2.AbstractButton.IconOnly
                onClicked: systems.create({
                        "system": systems.fresh(modelData),
                        "from": modelData
                    }, true)
            }

            QQC2.ToolButton {
                icon.name: "list-add"
                text: App.tr("Add a system that this one starts")
                display: QQC2.AbstractButton.IconOnly
                onClicked: systems.create({
                        "system": systems.fresh("child"),
                        "parent": modelData
                    }, true)
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove the system")
                display: QQC2.AbstractButton.IconOnly
                onClicked: systems.page.run("particles", {
                        "delete": {
                            "system": modelData
                        }
                    })
            }
        }
    }

    SceneSwarmNamed {
        Layout.fillWidth: true
        page: systems.page
        renamed: systems.renamed
        onDone: systems.renamed = null
    }

    QQC2.Button {
        icon.name: "list-add"
        text: App.tr("Add a system without a layer")
        onClicked: systems.create({
                "system": systems.fresh("system")
            }, true)
    }
}
