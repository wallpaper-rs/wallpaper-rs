import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    readonly property string stamp: JSON.stringify(page.view.scripts ?? [])
    property var listing: ({})
    readonly property var scripts: listing.scripts ?? []

    function reload() {
        panel.listing = JSON.parse(page.editor.code("list", "{}") || "{}");
    }

    onStampChanged: reload()
    Component.onCompleted: reload()

    Timer {
        interval: 1000
        repeat: true
        running: panel.visible && panel.scripts.length > 0
        onTriggered: panel.page.editor.watch()
    }

    SceneRepeater {
        id: list

        rows: panel.scripts

        delegate: SceneScriptRow {
            required property int index

            Layout.fillWidth: true
            page: panel.page
            script: list.rows[index]
            position: index
            total: list.rows.length
            onEdit: path => viewer.show(path)
        }
    }

    QQC2.ToolButton {
        icon.name: "list-add"
        text: App.tr("Attach a script of the folder…")
        onClicked: {
            panel.reload();
            picker.popup();
        }

        QQC2.Menu {
            id: picker

            QQC2.MenuItem {
                text: App.tr("Every script of the folder is already attached")
                enabled: false
                visible: (panel.listing.free ?? []).length === 0
                height: visible ? implicitHeight : 0
            }

            SceneRepeater {
                id: free

                rows: panel.listing.free ?? []

                delegate: QQC2.MenuItem {
                    required property int index

                    text: free.rows[index]
                    onTriggered: panel.page.run("script", {
                            "add": {
                                "path": free.rows[index]
                            }
                        })
                }
            }
        }
    }

    SceneScriptCode {
        id: viewer

        page: panel.page
    }
}
