import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Column {
    id: sublista

    property var model: []
    property string addText
    property string removeText: App.tr("Remove")
    property string otherText
    property string otherIcon
    property bool movable: false
    property int keep: -1

    signal removed(int index)
    signal added
    signal otherAdded
    signal reordered(int from, int to)

    function move(from, to) {
        if (!movable || to < 0 || to >= repeater.count || to === from)
            return;
        keep = to;
        reordered(from, to);
    }

    onModelChanged: keep >= 0 && Qt.callLater(() => {
        const item = repeater.itemAt(keep);
        keep = -1;
        if (item)
            item.forceActiveFocus(Qt.TabFocusReason);
    })

    bottomPadding: Look.e2
    Accessible.role: Accessible.List

    Repeater {
        id: repeater

        model: sublista.model

        FocusScope {
            id: item

            required property var modelData
            required property int index
            readonly property bool lit: activeFocus

            x: Look.filaLado
            width: sublista.width - Look.filaLado
            implicitHeight: Look.altoSublista
            activeFocusOnTab: true
            Accessible.role: Accessible.ListItem
            z: grip.active ? 1 : 0
            Accessible.name: [modelData.name, modelData.where ?? "", modelData.problem ?? ""].filter(part => part !== "").join(", ")
            Accessible.description: sublista.movable ? App.tr("Ctrl+Up and Ctrl+Down change its place") : ""
            Keys.onUpPressed: event => sublista.movable && event.modifiers & Qt.ControlModifier ? sublista.move(index, index - 1) : nextItemInFocusChain(false).forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: event => sublista.movable && event.modifiers & Qt.ControlModifier ? sublista.move(index, index + 1) : nextItemInFocusChain(true).forceActiveFocus(Qt.TabFocusReason)
            Keys.onDeletePressed: sublista.removeText !== "" && sublista.removed(index)
            transform: Translate {
                y: grip.active ? grip.translation.y : 0
            }

            Rectangle {
                anchors.fill: parent
                topRightRadius: Look.radio
                bottomRightRadius: Look.radio
                color: item.lit ? Look.escarcha : "transparent"
            }

            Rectangle {
                width: Look.linea
                height: parent.height
                color: item.lit ? Look.escarcha : Look.nocheLinea
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Look.e4
                anchors.rightMargin: Look.filaFin
                spacing: Look.listaHueco

                IconImage {
                    visible: sublista.movable
                    sourceSize.width: Look.icono
                    sourceSize.height: Look.icono
                    name: "handle-sort"
                    color: item.lit ? Look.noche : Look.bruma

                    DragHandler {
                        id: grip

                        target: null
                        xAxis.enabled: false
                        cursorShape: Qt.SizeVerCursor
                        onActiveChanged: active || sublista.move(item.index, Math.max(0, Math.min(repeater.count - 1, item.index + Math.round(translation.y / item.height))))
                    }
                }

                Foto {
                    visible: item.modelData.picture !== undefined
                    Layout.preferredWidth: Look.miniatura
                    Layout.preferredHeight: Look.miniatura * 9 / 16
                    corner: Look.radioChico
                    source: item.modelData.picture ?? ""
                    sourceSize.width: Look.miniatura * 2
                }

                Text {
                    Layout.maximumWidth: implicitWidth
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tCuerpo
                    color: item.lit ? Look.noche : Look.escarcha
                    textFormat: Text.PlainText
                    text: item.modelData.name
                }

                Text {
                    visible: (item.modelData.where ?? "") !== ""
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tPie
                    color: item.lit ? Look.noche : Look.bruma
                    text: "· " + (item.modelData.where ?? "")
                }

                Text {
                    visible: (item.modelData.problem ?? "") !== ""
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tPie
                    color: Tono.of("coral", item.lit, Look.coral)
                    text: "· " + (item.modelData.problem ?? "")
                }

                Item {
                    Layout.fillWidth: true
                }

                QQC2.Button {
                    id: remove

                    visible: sublista.removeText !== ""
                    flat: true
                    focusPolicy: Qt.NoFocus
                    text: sublista.removeText
                    icon.name: "x"
                    icon.color: item.lit ? Look.noche : Look.bruma
                    Accessible.name: sublista.removeText + " " + item.modelData.name
                    onClicked: sublista.removed(item.index)
                    contentItem: IconLabel {
                        spacing: Look.huecoBoton
                        icon: remove.icon
                        text: remove.text
                        font: remove.font
                        color: item.lit ? Look.noche : Look.escarcha
                    }
                }
            }
        }
    }

    Item {
        x: Look.filaLado
        visible: sublista.addText !== ""
        width: sublista.width - Look.filaLado
        implicitHeight: Look.altoSublista

        Rectangle {
            width: Look.linea
            height: parent.height
            color: Look.nocheLinea
        }

        QQC2.Button {
            x: Look.e4
            anchors.verticalCenter: parent.verticalCenter
            flat: true
            text: sublista.addText
            icon.name: "plus"
            Keys.onUpPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
            onClicked: sublista.added()
        }

        QQC2.Button {
            visible: sublista.otherText !== ""
            anchors.right: parent.right
            anchors.rightMargin: Look.filaFin
            anchors.verticalCenter: parent.verticalCenter
            flat: true
            text: sublista.otherText
            icon.name: sublista.otherIcon
            onClicked: sublista.otherAdded()
        }
    }
}
