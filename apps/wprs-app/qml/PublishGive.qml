import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: give

    property string text
    property string action
    property string copied: ""

    signal acted

    function copy() {
        field.text = copied;
        field.selectAll();
        field.copy();
        field.deselect();
        applicationWindow().showPassiveNotification(App.tr("Copied"), "short");
    }

    Layout.fillWidth: true
    implicitHeight: row.implicitHeight + 2 * Look.e3
    radius: Look.radio
    color: Look.nocheAlta
    border.width: Look.linea
    border.color: Look.nocheLinea

    RowLayout {
        id: row

        anchors.fill: parent
        anchors.margins: Look.e3
        spacing: Look.e3

        Text {
            Layout.fillWidth: true
            text: give.text
            elide: Text.ElideMiddle
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }

        QQC2.Button {
            text: give.action
            onClicked: give.copied !== "" ? give.copy() : give.acted()
        }
    }

    QQC2.TextField {
        id: field

        visible: false
        readOnly: true
    }
}
