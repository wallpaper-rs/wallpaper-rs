import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    readonly property var frames: JSON.parse(strips.frames || "{}")
    property alias strips: strips
    property alias cutter: cutter
    property alias packing: packing

    Strips {
        id: strips
    }

    Item {
        Layout.fillWidth: true
        Layout.fillHeight: true

        QQC2.ScrollView {
            anchors.fill: parent

            ListView {
                model: panel.page.view.assets ?? []
                clip: true

                delegate: SceneAssetRow {
                    owner: panel
                }
            }
        }

        SceneDropFiles {
            anchors.fill: parent
            page: panel.page
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

    SceneSpriteCutter {
        id: cutter

        page: panel.page
    }

    SceneImportSheet {
        id: sheet

        page: panel.page
    }

    FileDialog {
        id: files

        fileMode: FileDialog.OpenFiles
        nameFilters: [App.tr("Pictures, videos, sounds, fonts, color tables and models") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.ktx2 *.hdr *.exr *.webm *.mp4 *.mkv *.ogg *.opus *.flac *.wav *.mp3 *.ttf *.otf *.ttc *.otc *.woff2 *.cube *.glb)"]
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
