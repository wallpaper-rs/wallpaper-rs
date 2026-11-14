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
}
