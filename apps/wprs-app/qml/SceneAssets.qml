import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    function texture(t) {
        if (!t)
            return [];
        return [t.hdr ? App.tr("HDR") : "", t.cube ? App.tr("cube map, shown as a panorama") : "", t.frames > 1 ? App.tr("%1 frames", t.frames) : ""].filter(w => w);
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true

        ListView {
            model: panel.page.view.assets ?? []
            clip: true

            delegate: QQC2.ItemDelegate {
                id: row

                required property var modelData

                width: ListView.view.width
                text: [modelData.id, SceneWords.kind(modelData.kind), App.bytes(modelData.size)].concat(panel.texture(modelData.texture)).join(" · ") + (modelData.media ? " · " + (modelData.media === "current" ? App.tr("cover of what plays") : App.tr("previous cover")) : "")
                highlighted: panel.page.editor.selected === modelData.pointer
                onClicked: panel.page.editor.select(modelData.pointer)

                QQC2.ToolButton {
                    anchors.right: packed.left
                    icon.name: "edit-rename"
                    text: App.tr("Rename…")
                    display: QQC2.AbstractButton.IconOnly
                    onClicked: renaming.ask("asset", row.modelData.id)
                }

                QQC2.ToolButton {
                    id: packed

                    anchors.right: remove.left
                    visible: row.modelData.kind === "image" && !row.modelData.path.toLowerCase().endsWith(".ktx2")
                    icon.name: "application-x-archive"
                    text: App.tr("Convert to KTX2…")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: packing.show(row.modelData.id, row.modelData.pointer)
                }

                SceneRename {
                    id: renaming

                    page: panel.page
                }

                QQC2.ToolButton {
                    id: remove

                    anchors.right: parent.right
                    icon.name: "edit-delete"
                    text: App.tr("Remove")
                    display: QQC2.AbstractButton.IconOnly
                    onClicked: panel.page.run("remove", {
                            "pointer": row.modelData.pointer
                        })
                }
            }
        }
    }

    SceneAssetParts {
        Layout.fillWidth: true
        page: panel.page
    }

    Flow {
        Layout.fillWidth: true

        QQC2.ToolButton {
            text: App.tr("Import…")
            icon.name: "document-import"
            onClicked: files.open()
        }

        QQC2.ToolButton {
            text: App.tr("Import a sprite sheet or a data map…")
            icon.name: "view-grid"
            onClicked: sheet.open()
        }

        QQC2.ToolButton {
            text: App.tr("Import a script…")
            icon.name: "text-x-script"
            onClicked: modules.open()
        }

        QQC2.ToolButton {
            text: App.tr("New script…")
            icon.name: "document-new"
            onClicked: molds.popup()

            QQC2.Menu {
                id: molds

                Repeater {
                    model: [["empty", App.tr("Empty module")], ["media", App.tr("Module that follows the media player, with its events and values to follow")], ["touch", App.tr("Module that follows the pointer, with its position and the left button")], ["timer", App.tr("Module with a timer that counts seconds")], ["audio", App.tr("Module that follows the sound, with a smoothed level, the bass and the beat")], ["spawn", App.tr("Module that draws copies of a template of the scene, with settings")], ["wave", App.tr("Module with a swinging value and settings for its speed and size")], ["daytime", App.tr("Module that follows the time of day, with the daylight and the hour")], ["setting", App.tr("Module that follows a setting of the user, chosen with a setting of the module")], ["launches", App.tr("Module that counts how many times the scene started and keeps the count")]]

                    delegate: QQC2.MenuItem {
                        required property var modelData

                        text: modelData[1]
                        onTriggered: panel.page.run("import_script", {
                                "file": "",
                                "template": modelData[0]
                            })
                    }
                }
            }
        }

        QQC2.ToolButton {
            text: App.tr("Import from an assets package…")
            icon.name: "package-x-generic"
            onClicked: shared.open()
        }

        QQC2.ToolButton {
            text: App.tr("Make an assets package…")
            icon.name: "document-export"
            onClicked: offer.open()
        }

        QQC2.ToolButton {
            text: App.tr("Remove unused files")
            icon.name: "edit-clear-all"
            onClicked: panel.page.editor.prune()
        }
    }

    SceneShared {
        id: shared

        page: panel.page
    }

    SceneOffer {
        id: offer

        page: panel.page
    }

    SceneKtx2 {
        id: packing

        page: panel.page
    }

    SceneImportSheet {
        id: sheet

        page: panel.page
    }

    FileDialog {
        id: files

        fileMode: FileDialog.OpenFiles
        nameFilters: [App.tr("Pictures, videos, sounds, fonts, color tables and models") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.ktx2 *.hdr *.exr *.webm *.mp4 *.mkv *.ogg *.opus *.flac *.wav *.ttf *.otf *.cube *.glb)"]
        onAccepted: selectedFiles.forEach(file => panel.page.run("import", {
                        "file": App.path(file)
                    }))
    }

    FileDialog {
        id: modules

        fileMode: FileDialog.OpenFile
        nameFilters: [App.tr("A JavaScript module of the scene") + " (*.js)"]
        onAccepted: panel.page.run("import_script", {
                "file": App.path(selectedFile)
            })
    }
}
