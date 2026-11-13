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

    signal removed(int index)
    signal added

    bottomPadding: Look.e2
    Accessible.role: Accessible.List

    Repeater {
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
            Accessible.name: [modelData.name, modelData.where ?? "", modelData.problem ?? ""].filter(part => part !== "").join(", ")
            Keys.onUpPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
            Keys.onDeletePressed: sublista.removed(index)

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

                Text {
                    Layout.maximumWidth: implicitWidth
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tCuerpo
                    color: item.lit ? Look.noche : Look.escarcha
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
    }
}
