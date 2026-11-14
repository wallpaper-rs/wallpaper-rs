import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: frame

    required property var page
    required property string backWord
    readonly property alias seat: seat
    readonly property alias backButton: back
    readonly property bool own: !!(page && page.bar)
    readonly property point noticeAt: Qt.point(Look.lateralIzq + Look.e4, Look.arribaAlto + Look.barraLienzoAlto + Look.e4)

    signal leave

    Rectangle {
        id: bar

        width: frame.width
        height: Look.arribaAlto
        color: Look.noche

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Look.e4
            anchors.rightMargin: Look.e4
            spacing: Look.e4

            QQC2.Button {
                id: back

                flat: true
                icon.name: "caret-left"
                text: frame.backWord
                Accessible.name: App.tr("Back to %1", frame.backWord)
                onClicked: frame.leave()
            }

            Text {
                Layout.maximumWidth: frame.width / 3
                text: frame.page?.title ?? ""
                elide: Text.ElideRight
                color: Look.escarcha
                font.family: Look.letraTitulo
                font.pixelSize: Look.tArriba
                font.weight: Font.Bold
                Accessible.role: Accessible.Heading
                Accessible.name: text
            }

            Loader {
                Layout.fillWidth: true
                Layout.fillHeight: true
                active: frame.own
                visible: active
                sourceComponent: frame.own ? frame.page.bar : null
            }

            Kirigami.ActionToolBar {
                Layout.fillWidth: true
                visible: !frame.own
                alignment: Qt.AlignRight
                actions: frame.page?.actions ?? []
            }
        }

        Rectangle {
            anchors.bottom: parent.bottom
            width: parent.width
            height: Look.linea
            color: Look.nocheLinea
        }
    }

    Item {
        id: seat

        y: Look.arribaAlto
        width: frame.width
        height: frame.height - y
    }
}
