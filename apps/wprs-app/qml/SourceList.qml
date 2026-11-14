import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

FocusScope {
    id: list

    required property var providers
    required property var playlists
    readonly property int total: providers.reduce((sum, p) => sum + p.items, 0)

    signal rotate(var provider)
    signal details(var provider)
    signal fix(var provider)
    signal addAsked
    signal refreshAsked(string id)

    function enter() {
        const first = cards.itemAt(0);
        (first || adding).forceActiveFocus(Qt.TabFocusReason);
    }

    function rotating(provider) {
        return playlists.some(l => (l.query || "").includes("source:" + provider.source) && l.outputs.length > 0);
    }

    Flickable {
        id: flick

        anchors.fill: parent
        contentHeight: Math.max(height, column.implicitHeight)
        interactive: contentHeight > height
        clip: true

        Column {
            id: column

            anchors.bottom: parent.bottom
            width: Math.min(flick.width, Look.tiposAncho)
            spacing: Look.e4

            Row {
                spacing: Look.e3

                Filtro {
                    words: [{
                            "text": App.trc("all the sources", "All"),
                            "count": list.providers.length
                        }]
                    anchors.verticalCenter: parent.verticalCenter
                }

                QQC2.Button {
                    flat: true
                    enabled: list.providers.length > 0
                    text: App.tr("Update all")
                    icon.name: "arrows-clockwise"
                    onClicked: list.refreshAsked("")
                }

                QQC2.Button {
                    id: adding

                    highlighted: list.providers.length === 0
                    text: App.tr("Add a source")
                    icon.name: "plus"
                    onClicked: list.addAsked()
                    Keys.onDownPressed: list.enter()
                }
            }

            Repeater {
                id: cards

                model: list.providers

                delegate: SourceCard {
                    required property var modelData
                    required property int index

                    width: column.width
                    provider: modelData
                    rotating: list.rotating(modelData)
                    onRotate: list.rotate(modelData)
                    onRefresh: list.refreshAsked(modelData.id)
                    onDetails: list.details(modelData)
                    onFix: list.fix(modelData)
                    onUp: index > 0 ? cards.itemAt(index - 1).enter() : adding.forceActiveFocus(Qt.TabFocusReason)
                    onDown: index < cards.count - 1 && cards.itemAt(index + 1).enter()
                }
            }

            QQC2.Button {
                flat: true
                text: App.tr("Your own photos? Add a folder to your library")
                icon.name: "plus"
                onClicked: folder.open()
            }
        }
    }

    FolderDialog {
        id: folder

        onAccepted: App.act("AddSource", [App.path(selectedFolder), true], "Library1")
    }
}
