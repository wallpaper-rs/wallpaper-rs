import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: place

    property string icon
    property string title
    property string note
    property bool lit: false
    property bool off: false
    property bool last: false
    default property alias body: content.data

    signal picked

    Layout.fillWidth: true
    implicitHeight: column.implicitHeight + 2 * Look.e3
    radius: Look.radio
    color: lit ? Look.nocheAlta : Look.noche
    border.width: Look.linea
    border.color: lit ? Look.escarcha : Look.nocheLinea
    opacity: off ? 0.6 : 1
    activeFocusOnTab: !off
    Accessible.role: Accessible.RadioButton
    Accessible.name: title
    Accessible.description: note
    Accessible.checked: lit
    Keys.onSpacePressed: !off && picked()

    TapHandler {
        enabled: !place.off
        onTapped: place.picked()
    }

    Anillo {
        corner: place.radius
        visible: place.activeFocus
    }

    ColumnLayout {
        id: column

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: Look.e3
        spacing: Look.e2

        RowLayout {
            Layout.fillWidth: true
            spacing: Look.e3

            IconImage {
                name: place.icon
                sourceSize: Qt.size(Look.iconoDestino, Look.iconoDestino)
                color: place.lit ? Look.escarcha : Look.bruma
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 0

                Text {
                    Layout.fillWidth: true
                    text: place.title
                    elide: Text.ElideRight
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tCuerpo
                    font.weight: Font.DemiBold
                    color: Look.escarcha
                }

                Text {
                    Layout.fillWidth: true
                    visible: place.note !== ""
                    text: place.note
                    wrapMode: Text.WordWrap
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tPie
                    color: Look.bruma
                }
            }

            Text {
                visible: place.last
                text: App.tr("Last time")
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                color: Look.brote
            }
        }

        ColumnLayout {
            id: content

            Layout.fillWidth: true
            Layout.leftMargin: Look.iconoDestino + Look.e3
            visible: place.lit && children.length > 0
            spacing: Look.e2
        }
    }
}
