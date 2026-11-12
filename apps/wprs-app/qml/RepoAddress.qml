import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: address

    property string text: ""
    property bool copied: false
    readonly property alias field: field

    function copy() {
        field.selectAll();
        field.copy();
        field.deselect();
        copied = true;
    }

    visible: text !== ""
    spacing: Kirigami.Units.smallSpacing
    onTextChanged: copied = false

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: App.tr("Address to add this repository. Give it to whoever installs from it. If you publish the folder on the web, replace the start, up to the question mark, with its web address.")
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            id: field

            Layout.fillWidth: true
            readOnly: true
            text: address.text
            Accessible.name: App.tr("Repository address")
        }

        QQC2.Button {
            icon.name: "edit-copy"
            text: address.copied ? App.tr("Copied") : App.tr("Copy the address")
            onClicked: address.copy()
        }
    }
}
