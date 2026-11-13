import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: root

    required property var timeline
    property string renaming: ""
    readonly property alias naming: rename

    function fresh() {
        const list = timeline.page.view.timelines ?? [];
        let n = list.length + 1;
        while (list.some(line => line.id === "timeline_" + n))
            n++;
        return "timeline_" + n;
    }

    Instantiator {
        model: root.timeline.page.view.timelines ?? []
        onObjectAdded: (index, object) => root.insertMenu(index, object)
        onObjectRemoved: (index, object) => root.removeMenu(object)

        delegate: QQC2.Menu {
            id: line

            required property var modelData

            title: modelData.id + " · " + App.tr("%1 s", modelData.duration)

            QQC2.MenuItem {
                text: App.tr("Edit the timeline and its events…")
                icon.name: "document-edit"
                onTriggered: {
                    editing.pointer = line.modelData.pointer;
                    editing.open();
                }
            }

            QQC2.MenuItem {
                text: App.tr("Show in the inspector")
                icon.name: "document-properties"
                onTriggered: root.timeline.page.editor.select(line.modelData.pointer)
            }

            QQC2.MenuItem {
                text: App.tr("Rename…")
                icon.name: "edit-rename"
                onTriggered: {
                    root.renaming = line.modelData.id;
                    named.text = line.modelData.id;
                    rename.open();
                }
            }

            QQC2.MenuItem {
                text: App.tr("Remove; the values it drives follow the scene clock")
                icon.name: "edit-delete"
                onTriggered: root.timeline.page.run("timeline", {
                    "id": line.modelData.id,
                    "remove": true
                })
            }
        }
    }

    QQC2.MenuItem {
        text: App.tr("Add a timeline")
        icon.name: "list-add"
        onTriggered: root.timeline.page.run("timeline", {
            "id": root.fresh(),
            "add": true,
            "duration": 5,
            "loop": "repeat"
        })
    }

    SceneTimelinePanel {
        id: editing

        parent: QQC2.Overlay.overlay
        page: root.timeline.page
    }

    Kirigami.PromptDialog {
        id: rename

        parent: QQC2.Overlay.overlay
        title: App.tr("Rename the timeline")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: root.timeline.page.run("timeline", {
            "id": root.renaming,
            "to": named.text
        })

        QQC2.TextField {
            id: named

            validator: RegularExpressionValidator {
                regularExpression: /[a-z0-9_]{1,32}/
            }
        }
    }
}
