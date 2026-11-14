import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

FocusScope {
    id: cards

    property bool compact: false
    property var groups: [{
        "text": App.tr("With one click, no account"),
        "kinds": SourceKinds.catalog.filter(k => k.quick)
    }, {
        "text": App.tr("With your search or your server"),
        "kinds": SourceKinds.catalog.filter(k => !k.quick)
    }]
    readonly property var flat: groups.reduce((all, g) => all.concat(g.kinds), [])
    property int at: 0
    readonly property bool keyTarget: true

    signal picked(string kind)
    signal closed

    function enter() {
        forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: column.implicitHeight
    implicitWidth: Look.tiposAncho
    Keys.onEscapePressed: closed()
    Keys.onLeftPressed: at = Math.max(0, at - 1)
    Keys.onRightPressed: at = Math.min(flat.length - 1, at + 1)
    Keys.onUpPressed: at = Math.max(0, at - 2)
    Keys.onDownPressed: at = Math.min(flat.length - 1, at + 2)
    Keys.onReturnPressed: picked(flat[at].kind)
    Keys.onEnterPressed: picked(flat[at].kind)

    Column {
        id: column

        width: parent.width
        spacing: Look.e4

        Repeater {
            model: cards.groups

            delegate: Column {
                id: group

                required property var modelData
                required property int index
                readonly property int base: index === 0 ? 0 : cards.groups[0].kinds.length

                width: column.width
                spacing: Look.e2

                Seccion {
                    text: group.modelData.text
                }

                Flow {
                    width: parent.width
                    spacing: Look.e3

                    Repeater {
                        model: group.modelData.kinds

                        delegate: Rectangle {
                            id: kindCard

                            required property var modelData
                            required property int index
                            readonly property bool lit: cards.activeFocus && cards.at === group.base + index

                            width: (column.width - Look.e3) / 2
                            height: Look.altoFila
                            radius: Look.radioChico
                            color: lit ? Look.escarcha : Look.nocheAlta
                            border.width: 1
                            border.color: lit ? Look.escarcha : Look.nocheLinea

                            Row {
                                anchors.fill: parent
                                anchors.margins: Look.e3
                                spacing: Look.e3

                                IconImage {
                                    anchors.verticalCenter: parent.verticalCenter
                                    name: kindCard.modelData.icon
                                    sourceSize: Qt.size(Look.tipoIcono, Look.tipoIcono)
                                    color: kindCard.lit ? Look.noche : Look.luna
                                }

                                Column {
                                    anchors.verticalCenter: parent.verticalCenter

                                    Text {
                                        text: kindCard.modelData.line
                                        font.family: Look.letraTexto
                                        font.pixelSize: Look.tCuerpo
                                        font.weight: Font.Medium
                                        color: kindCard.lit ? Look.noche : Look.escarcha
                                    }

                                    Text {
                                        text: kindCard.modelData.name
                                        font.family: Look.letraTexto
                                        font.pixelSize: Look.tPie
                                        color: kindCard.lit ? Look.noche : Look.bruma
                                    }
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: {
                                    cards.at = group.base + kindCard.index;
                                    cards.picked(kindCard.modelData.kind);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
