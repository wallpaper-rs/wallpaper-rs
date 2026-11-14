import QtQuick
import io.github.wallpaper_rs.app

Coleccion {
    id: estante

    property Component actions
    property Component selection
    property alias currentIndex: list.currentIndex
    readonly property real place: Look.filete + Look.anillo + Look.tapa + Look.huecoEstante

    view: list
    implicitHeight: Math.max(list.currentItem?.height ?? 0, Math.round(Look.tapaFoco * 9 / 16) + Look.e7 + Look.e4 + Look.filete + Look.anillo)
    Accessible.role: Accessible.List
    Accessible.name: label

    Item {
        x: -Look.margen
        width: parent.width + Look.margen
        height: parent.height
        clip: true

        ListView {
            id: list

            x: Look.margen
            width: parent.width - Look.margen
            height: parent.height
            orientation: ListView.Horizontal
            spacing: Look.huecoEstante
            leftMargin: Look.filete + Look.anillo
            rightMargin: Math.max(Look.filete + Look.anillo, width - estante.place - Look.tapaFoco)
            displayMarginBeginning: Look.margen
            focus: true
            reuseItems: true
            cacheBuffer: Look.tapa
            boundsBehavior: Flickable.StopAtBounds
            highlightFollowsCurrentItem: true
            highlightMoveDuration: Look.paso
            highlightRangeMode: ListView.ApplyRange
            preferredHighlightBegin: estante.place
            preferredHighlightEnd: estante.place + Look.tapaFoco + Look.filete + Look.anillo
            highlight: Item {
            }
            model: estante.model
            Keys.onPressed: event => estante.press(event)
            Component.onCompleted: Qt.callLater(() => {
                positionViewAtBeginning();
                positionViewAtIndex(currentIndex, ListView.Contain);
            })

            delegate: Item {
                id: slot

                required property var model
                required property int index
                readonly property alias name: cover.name
                readonly property alias caption: cover.caption
                readonly property real room: cover.room

                width: cover.width + (cover.lit ? Look.filete + Look.anillo : 0)
                height: cover.y + cover.height
                z: cover.lit ? 1 : 0

                TapaVista {
                    id: cover

                    y: Look.filete + Look.anillo + Math.round((Look.tapaFoco - width) * 9 / 16)
                    width: implicitWidth
                    height: implicitHeight
                    model: slot.model
                    index: slot.index
                    from: estante

                    Loader {
                        readonly property bool choosing: estante.choosing && estante.chosen.length > 0

                        active: cover.lit
                        x: choosing || !item ? 0 : Math.min(0, list.width - Look.filete - Look.anillo - (slot.x - list.contentX) - item.width)
                        width: choosing && item ? Math.min(item.implicitWidth, list.width - Look.margen - (slot.x - list.contentX)) : undefined
                        sourceComponent: choosing ? apart : card
                    }
                }
            }
        }
    }

    Component {
        id: apart

        Item {
            implicitWidth: chosenRow.implicitWidth
            implicitHeight: chosenRow.y + chosenRow.implicitHeight

            Loader {
                id: chosenRow

                y: list.currentItem?.room ?? 0
                width: parent.width
                sourceComponent: estante.selection
            }
        }
    }

    Component {
        id: card

        Ficha {
            title: list.currentItem?.name ?? ""
            caption: list.currentItem?.caption ?? ""

            Loader {
                sourceComponent: estante.actions
            }
        }
    }
}
