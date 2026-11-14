import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Rectangle {
    id: step

    property int number
    property string title
    property string done
    property string mode: "cerrado"
    property bool changeable: true
    readonly property bool open: mode === "abierto"
    readonly property alias changer: change
    default property alias body: content.data
    property alias after: tail.data

    signal change

    function plain(text) {
        return String(text).replace(/<[^>]*>/g, "");
    }

    Layout.fillWidth: true
    implicitHeight: column.implicitHeight + 2 * (open ? Look.e4 : Look.e3)
    radius: Look.radio
    color: Look.noche
    border.width: Look.linea
    border.color: open ? Look.brumaBaja : Look.nocheLinea
    opacity: mode === "cerrado" ? 0.7 : 1
    Accessible.role: Accessible.Grouping
    Accessible.name: mode === "hecho" ? step.plain(done) : title

    ColumnLayout {
        id: column

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.leftMargin: Look.e5
        anchors.rightMargin: Look.e5
        anchors.topMargin: step.open ? Look.e4 : Look.e3
        spacing: Look.e3

        RowLayout {
            Layout.fillWidth: true
            spacing: Look.e4

            Item {
                Layout.preferredWidth: PublishLook.pasoLado
                Layout.preferredHeight: PublishLook.pasoNum

                Rectangle {
                    width: PublishLook.pasoNum
                    height: PublishLook.pasoNum
                    radius: PublishLook.pasoNum / 2
                    color: step.open ? Look.escarcha : "transparent"
                    border.width: Look.linea
                    border.color: step.open ? Look.escarcha : Look.nocheLinea

                    Text {
                        anchors.centerIn: parent
                        visible: step.mode !== "hecho"
                        text: step.number
                        font.family: Look.letraTitulo
                        font.weight: Font.Bold
                        font.pixelSize: Look.tCuerpo
                        color: step.open ? Look.noche : Look.bruma
                    }

                    IconImage {
                        anchors.centerIn: parent
                        visible: step.mode === "hecho"
                        name: "check-bold"
                        sourceSize: Qt.size(Look.iconoNota, Look.iconoNota)
                        color: Look.brote
                    }
                }
            }

            Text {
                Layout.fillWidth: true
                text: step.mode === "hecho" ? step.done : step.title
                textFormat: step.mode === "hecho" ? Text.StyledText : Text.PlainText
                elide: Text.ElideRight
                font.family: step.mode === "hecho" ? Look.letraTexto : Look.letraTitulo
                font.pixelSize: step.mode === "hecho" ? Look.tCuerpo : Look.tSeccion
                font.weight: step.mode === "hecho" ? Font.Medium : Font.Bold
                color: step.mode === "hecho" ? Look.bruma : Look.escarcha
                linkColor: Look.escarcha
            }

            QQC2.Button {
                id: change

                visible: step.mode === "hecho" && step.changeable
                flat: true
                text: App.tr("Change")
                Accessible.name: App.tr("Change: %1", step.plain(step.done))
                onClicked: step.change()
            }
        }

        ColumnLayout {
            id: content

            Layout.fillWidth: true
            Layout.leftMargin: PublishLook.pasoLado + Look.e4
            visible: step.open
            spacing: Look.e3
        }

        ColumnLayout {
            id: tail

            Layout.fillWidth: true
            Layout.leftMargin: PublishLook.pasoLado + Look.e4
            visible: step.mode === "hecho" && children.length > 0
            spacing: Look.e2
        }
    }
}
