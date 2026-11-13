import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: offered

    required property var host
    readonly property var page: host.page
    property var parts: []
    property var picking: null

    function implicit(scene) {
        const named = (kind, list) => list.map(id => ({
                    "kind": kind,
                    "id": id
                }));
        return named("layer", (scene.layers ?? []).map(l => l.id)).concat(named("particles", Object.keys(scene.particles ?? {})), named("script", (scene.scripts ?? []).map(s => s.path)), named("effect", Object.keys(scene.effects ?? {})));
    }

    function load() {
        const scene = JSON.parse(page.editor.value("/scene") || "{}") ?? ({});
        parts = scene.ext?.parts ?? implicit(scene);
    }

    function change(part, args) {
        page.manifestEdit("part", Object.assign({
                "kind": part.kind,
                "id": part.id
            }, args));
    }

    spacing: Kirigami.Units.smallSpacing
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            offered.load();
        }

        target: offered.page.editor
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("Each part has a title and a thumbnail that whoever imports it sees.")
    }

    Repeater {
        model: offered.parts

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                elide: Text.ElideRight
                textFormat: Text.PlainText
                text: row.modelData.kind + ": " + row.modelData.id
                Layout.preferredWidth: Kirigami.Units.gridUnit * 10
            }

            QQC2.TextField {
                Layout.fillWidth: true
                maximumLength: 128
                text: row.modelData.title ?? ""
                placeholderText: App.tr("Title")
                Accessible.name: App.tr("Title of the part")
                onEditingFinished: if (text !== (row.modelData.title ?? ""))
                    offered.change(row.modelData, {
                        "title": text
                    })
            }

            QQC2.Button {
                text: row.modelData.thumbnail ? App.tr("Change the thumbnail…") : App.tr("Choose a thumbnail…")
                icon.name: "document-open"
                onClicked: {
                    offered.picking = row.modelData;
                    chooser.open();
                }
            }

            QQC2.Button {
                visible: !!row.modelData.thumbnail
                icon.name: "edit-delete"
                QQC2.ToolTip.text: App.tr("Remove the thumbnail")
                QQC2.ToolTip.visible: hovered
                Accessible.name: App.tr("Remove the thumbnail")
                onClicked: offered.change(row.modelData, {
                    "clear_thumbnail": true
                })
            }
        }
    }

    FileDialog {
        id: chooser

        nameFilters: [App.tr("Pictures (*.png *.jpg *.jpeg *.webp)")]
        onAccepted: offered.change(offered.picking, {
            "thumbnail": App.path(selectedFile)
        })
    }
}
