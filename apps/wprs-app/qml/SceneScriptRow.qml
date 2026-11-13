import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: row

    required property var page
    required property var script
    required property int position
    required property int total
    property bool open: false
    property real dragged: 0

    signal edit(string path)

    function drop(offset) {
        const to = Math.max(0, Math.min(total - 1, position + Math.round(offset / Math.max(height, 1))));
        if (to !== position)
            move(to);
    }

    function move(to) {
        page.run("script", {
                "move": {
                    "path": script.path,
                    "to": to
                }
            });
    }

    z: grip.dragging ? 1 : 0
    spacing: 0
    transform: Translate {
        y: row.dragged
    }

    RowLayout {
        Layout.fillWidth: true

        Kirigami.Icon {
            id: grip

            readonly property bool dragging: drag.active

            source: "handle-sort"
            implicitWidth: Kirigami.Units.iconSizes.small
            implicitHeight: implicitWidth
            QQC2.ToolTip.text: App.tr("Drag to change the order")
            QQC2.ToolTip.visible: grab.hovered
            HoverHandler {
                id: grab

                cursorShape: Qt.SizeVerCursor
            }

            DragHandler {
                id: drag

                xAxis.enabled: false
                onTranslationChanged: {
                    if (active)
                        row.dragged = translation.y;
                }
                onActiveChanged: {
                    if (!active) {
                        const offset = row.dragged;
                        row.dragged = 0;
                        row.drop(offset);
                    }
                }
            }
        }

        QQC2.ToolButton {
            icon.name: row.open ? "arrow-down" : "arrow-right"
            text: row.open ? App.tr("Hide the declaration and settings") : App.tr("Show the declaration and settings")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: row.open = !row.open
        }

        QQC2.Label {
            Layout.fillWidth: true
            elide: Text.ElideMiddle
            text: [row.script.path, row.script.missing ? App.tr("missing") : App.bytes(row.script.size), row.script.bound.length > 0 ? App.tr("followed by %1", row.script.bound.length) : ""].filter(part => part !== "").join(" · ")
            color: row.script.missing ? Look.coral : Kirigami.Theme.textColor
        }

        Kirigami.Icon {
            visible: !row.script.synced
            source: "dialog-warning"
            implicitWidth: Kirigami.Units.iconSizes.small
            implicitHeight: implicitWidth
            QQC2.ToolTip.text: App.tr("What the code uses differs from what the scene declares")
            QQC2.ToolTip.visible: hint.hovered
            HoverHandler {
                id: hint
            }
        }

        QQC2.ToolButton {
            icon.name: "go-up"
            text: App.tr("Move up")
            enabled: row.position > 0
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: row.move(row.position - 1)
        }

        QQC2.ToolButton {
            icon.name: "go-down"
            text: App.tr("Move down")
            enabled: row.position < row.total - 1
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: row.move(row.position + 1)
        }

        QQC2.ToolButton {
            icon.name: "document-edit"
            text: App.tr("Edit the code…")
            enabled: !row.script.missing
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: row.edit(row.script.path)
        }

        QQC2.ToolButton {
            icon.name: "document-open"
            text: App.tr("Open in your editor…")
            enabled: !row.script.missing
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: Qt.openUrlExternally("file://" + row.page.editor.folder + "/" + row.script.path)
        }

        QQC2.ToolButton {
            icon.name: "edit-delete"
            text: App.tr("Remove")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: row.page.run("script", {
                    "remove": {
                        "path": row.script.path
                    }
                })
        }
    }

    ColumnLayout {
        visible: row.open
        Layout.fillWidth: true
        Layout.leftMargin: Kirigami.Units.largeSpacing

        SceneScriptAudit {
            Layout.fillWidth: true
            page: row.page
            path: row.script.path
            report: row.script.audit
        }

        SceneScriptParams {
            Layout.fillWidth: true
            page: row.page
            script: row.script
        }
    }
}
