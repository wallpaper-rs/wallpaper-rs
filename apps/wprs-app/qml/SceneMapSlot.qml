import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: slot

    required property var owner
    required property var map
    readonly property var page: owner.panel.page
    readonly property var chosen: (owner.material.chosen ?? []).find(c => c.slot === map.slot) ?? null
    readonly property var pictures: (page.view.assets ?? []).filter(a => a.kind === "image")

    function choose(id) {
        const maps = {};
        maps[map.slot] = id;
        owner.panel.set(owner.material.name, {
            "maps": maps
        });
    }

    function url(asset) {
        return asset && asset.path ? App.url(page.editor.folder + "/" + asset.path) : "";
    }

    QQC2.ToolButton {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 3
        Layout.preferredHeight: Layout.preferredWidth
        QQC2.ToolTip.text: App.tr("Choose or replace this map")
        QQC2.ToolTip.visible: hovered
        onClicked: menu.popup()

        Image {
            anchors.fill: parent
            anchors.margins: 2
            fillMode: Image.PreserveAspectCrop
            source: slot.chosen ? slot.url(slot.chosen) : (slot.map.thumb ?? "")
            visible: source !== ""
        }
    }

    QQC2.Label {
        text: slot.owner.names[slot.map.slot] + (slot.chosen ? " · " + slot.chosen.asset : slot.map.thumb === null ? " " + App.tr("(none)") : "")
        font: Kirigami.Theme.smallFont
    }

    QQC2.Label {
        visible: slot.chosen?.warn ?? false
        text: App.tr("Read as colors: import it as data")
        color: Kirigami.Theme.neutralTextColor
        font: Kirigami.Theme.smallFont
    }

    QQC2.Menu {
        id: menu

        QQC2.MenuItem {
            text: App.tr("The map of the file")
            enabled: slot.chosen !== null
            onTriggered: slot.choose(null)
        }

        QQC2.MenuItem {
            icon.name: "document-open"
            text: App.tr("Picture from a file…")
            onTriggered: files.open()
        }

        QQC2.MenuSeparator {}

        Repeater {
            model: slot.pictures

            QQC2.MenuItem {
                required property var modelData

                text: modelData.id
                icon.source: slot.url(modelData)
                icon.color: "transparent"
                checkable: true
                checked: slot.chosen?.asset === modelData.id
                onTriggered: slot.choose(modelData.id)
            }
        }
    }

    FileDialog {
        id: files

        nameFilters: [App.tr("Pictures") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.ktx2 *.hdr *.exr)"]
        onAccepted: {
            const given = {};
            given[slot.map.slot] = App.path(selectedFile);
            slot.owner.panel.set(slot.owner.material.name, {
                "files": given
            });
        }
    }
}
