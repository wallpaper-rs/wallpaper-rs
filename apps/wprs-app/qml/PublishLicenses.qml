import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

ColumnLayout {
    id: licenses

    required property var review
    readonly property bool shown: review.license === "" || picking
    property bool picking: false

    visible: shown
    spacing: Look.e1

    Text {
        text: App.tr("License")
        font.family: Look.letraTitulo
        font.pixelSize: Look.tAparte
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    Repeater {
        model: Publishing.licenses

        delegate: Opcion {
            required property var modelData

            Layout.fillWidth: true
            icon: licenses.review.license === modelData.id ? "radio-button-fill" : "circle"
            lit: licenses.review.license === modelData.id
            text: modelData.name + ": " + modelData.says
            onClicked: {
                licenses.review.publish.page.change("/manifest/license", modelData.id);
                licenses.picking = false;
                licenses.review.refresh();
            }
        }
    }
}
