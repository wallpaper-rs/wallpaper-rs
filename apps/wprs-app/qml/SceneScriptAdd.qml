import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Flow {
    id: add

    required property var page

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
                        onTriggered: add.page.run("import_script", {
                                "file": "",
                                "template": modelData[0]
                            })
                    }
                }
            }
        }

    FileDialog {
        id: modules

        fileMode: FileDialog.OpenFile
        nameFilters: [App.tr("A JavaScript module of the scene") + " (*.js)"]
        onAccepted: add.page.run("import_script", {
                "file": App.path(selectedFile)
            })
    }
}
