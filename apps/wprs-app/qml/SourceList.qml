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
    readonly property bool offline: providers.some(p => p.offline)
    property bool wide: false
    readonly property real tallest: column.implicitHeight + Look.e3 + own.height
    readonly property real lowest: Math.min(tallest, Look.altoBoton * 5 + Look.e4 + Look.e3)

    signal rotate(var provider)
    signal details(var provider)
    signal fix(var provider)
    signal addAsked
    signal refreshAsked(string id)

    function enter() {
        const first = cards.itemAt(0);
        (first || adding).forceActiveFocus(Qt.TabFocusReason);
    }

    function card(id) {
        return cards.itemAt(providers.findIndex(p => p.id === id));
    }

    function rotating(provider) {
        return playlists.some(l => (l.query || "").includes("source:" + provider.source) && l.outputs.length > 0);
    }

    Flickable {
        id: flick

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.bottom: own.top
        anchors.bottomMargin: Look.e3
        contentHeight: Math.max(height, column.implicitHeight)
        interactive: contentHeight > height
        clip: true

        Column {
            id: column

            anchors.bottom: parent.bottom
            width: list.wide ? flick.width : Math.min(flick.width, Look.tiposAncho)
            spacing: Look.e4

            Flow {
                visible: list.offline
                width: column.width
                spacing: Look.e3

                Nota {
                    height: again.height
                    tone: "ambar"
                    text: App.tr("No connection. Your sources cannot bring new photos. What you already have stays in your library.")
                }

                QQC2.Button {
                    id: again

                    text: App.tr("Try again")
                    icon.name: "arrows-clockwise"
                    onClicked: list.refreshAsked("")
                }
            }

            Item {
                readonly property bool wraps: filter.width + Look.e3 + buttons.width > width

                width: column.width
                height: wraps ? filter.height + Look.e3 + buttons.height : buttons.height

                Filtro {
                    id: filter

                    words: [{
                            "text": App.trc("all the sources", "All"),
                            "count": list.providers.length
                        }]
                    y: parent.wraps ? 0 : (parent.height - height) / 2
                }

                Row {
                    id: buttons

                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    spacing: Look.e3

                    QQC2.Button {
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
        }
    }

    QQC2.Button {
        id: own

        anchors.left: parent.left
        anchors.bottom: parent.bottom
        flat: true
        text: App.tr("Your own photos? Add a folder to your library")
        icon.name: "plus"
        onClicked: folder.open()
    }

    FolderDialog {
        id: folder

        onAccepted: App.act("AddSource", [App.path(selectedFolder), true], "Library1")
    }
}
