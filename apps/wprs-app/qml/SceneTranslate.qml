import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: row

    required property var host
    property bool keys: false

    Layout.fillWidth: true
    visible: host.page.view.v2 !== false

    QQC2.TextField {
        id: tag

        Layout.fillWidth: true
        placeholderText: App.tr("Language, such as es or pt-BR")
        onAccepted: add.clicked()
    }

    Loader {
        active: row.keys

        sourceComponent: TextsKeys {
            host: row.host
        }
    }

    QQC2.Button {
        id: add

        icon.name: "list-add"
        text: App.tr("Translate")
        enabled: /^[A-Za-z]{2,3}(-[A-Za-z0-9]{1,8}){0,3}$/.test(tag.text)
        onClicked: {
            row.host.page.run("translate", {
                    "pointer": row.host.field.pointer,
                    "language": tag.text,
                    "add": true
                });
            tag.clear();
        }
    }
}
