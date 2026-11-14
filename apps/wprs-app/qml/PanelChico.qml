import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Popup {
    id: panel

    property Item opener
    property url cover
    property string title
    property string text
    property string quiet
    property string principal
    property string principalIcon
    property bool borra: false
    property bool padded: true
    property Item first: principal !== "" ? main : quiet !== "" ? calm : null
    default property alias content: dentro.data
    readonly property alias foot: extra.data

    signal accepted
    signal rejected

    function place() {
        if (!opener || !parent)
            return;
        const below = opener.mapToItem(parent, 0, opener.height + Look.e2);
        const above = opener.mapToItem(parent, 0, -Look.e2).y - implicitHeight;
        x = below.x;
        y = below.y + implicitHeight + Look.e4 > parent.height && above >= Look.e4 ? above : below.y;
    }

    parent: QQC2.Overlay.overlay
    width: Look.panelChico
    padding: 0
    margins: Look.e4
    focus: true
    closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutside
    onAboutToShow: place()
    onOpened: (first ?? body).forceActiveFocus(Qt.TabFocusReason)
    onClosed: opener?.forceActiveFocus(Qt.TabFocusReason)

    background: Rectangle {
        radius: Look.radioPanel
        color: Look.noche
        border.width: Look.linea
        border.color: Look.nocheLinea

        Sombra {
            corner: Look.radioPanel
            kind: "panel"
        }
    }

    contentItem: FocusScope {
        id: body

        implicitWidth: panel.width
        implicitHeight: column.implicitHeight
        Accessible.role: Accessible.Dialog
        Accessible.name: panel.title

        Column {
            id: column

            width: parent.width

            Foto {
                visible: panel.cover.toString() !== ""
                width: parent.width
                height: width * 7 / 16
                source: panel.cover
                corner: Look.radioPanel
                lowCorner: 0
                sourceSize.width: width

                Rectangle {
                    anchors.fill: parent
                    gradient: Gradient {
                        GradientStop {
                            position: 0.4
                            color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0)
                        }
                        GradientStop {
                            position: 1
                            color: Look.noche
                        }
                    }
                }
            }

            Column {
                id: dentro

                width: parent.width
                padding: panel.padded ? Look.e4 : 0
                spacing: Look.e3

                Text {
                    visible: panel.title !== ""
                    width: parent.width - 2 * parent.padding
                    text: panel.title
                    wrapMode: Text.WordWrap
                    font.family: Look.letraTitulo
                    font.pixelSize: Look.tFila
                    font.weight: Font.Bold
                    color: Look.escarcha
                }

                Text {
                    visible: panel.text !== ""
                    width: parent.width - 2 * parent.padding
                    text: panel.text
                    wrapMode: Text.WordWrap
                    lineHeight: 1.3
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tCuerpo
                    color: Look.bruma
                }
            }

            Rectangle {
                visible: panel.quiet !== "" || panel.principal !== "" || extra.children.length > 0
                width: parent.width
                height: Look.linea
                color: Look.nocheLinea
            }

            RowLayout {
                visible: panel.quiet !== "" || panel.principal !== "" || extra.children.length > 0
                width: parent.width - 2 * Look.e4
                x: Look.e4
                height: implicitHeight + 2 * Look.e3
                spacing: Look.e2

                Row {
                    id: extra

                    Layout.fillWidth: true
                    spacing: Look.e2
                }

                QQC2.Button {
                    id: calm

                    visible: panel.quiet !== ""
                    text: panel.quiet
                    onClicked: {
                        panel.rejected();
                        panel.close();
                    }
                }

                QQC2.Button {
                    id: main

                    visible: panel.principal !== ""
                    highlighted: true
                    palette.highlight: panel.borra ? Look.coral : Look.luna
                    text: panel.principal
                    icon.name: panel.principalIcon
                    onClicked: {
                        panel.accepted();
                        panel.close();
                    }
                }
            }
        }
    }
}
