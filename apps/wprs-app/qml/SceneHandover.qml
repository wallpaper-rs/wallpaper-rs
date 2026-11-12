import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: handover

    required property var keying

    function sign() {
        keying.backend.rotate(keying.chosen, oldPassword.text, newKey.text, author.text);
        oldPassword.text = "";
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Hand an online author over to a new key")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: App.tr("Sign with the key that signed the earlier versions; the server accepts the new public key for that author.")
    }

    Kirigami.FormLayout {
        Layout.fillWidth: true

        Kirigami.PasswordField {
            id: oldPassword

            Kirigami.FormData.label: App.tr("Password of the old key:")
            enabled: keying.chosen !== ""
        }

        RowLayout {
            Kirigami.FormData.label: App.tr("New public key:")

            QQC2.TextField {
                id: newKey

                Layout.fillWidth: true
            }

            QQC2.Button {
                icon.name: "document-open"
                text: App.tr("Choose…")
                onClicked: publics.open()
            }
        }

        QQC2.TextField {
            id: author

            Kirigami.FormData.label: App.tr("Author id:")
        }

        QQC2.Button {
            text: App.tr("Sign the hand-over")
            icon.name: "document-sign"
            enabled: keying.chosen !== "" && newKey.text !== "" && author.text !== "" && !keying.backend.busy
            onClicked: handover.sign()
        }

        Area {
            Kirigami.FormData.label: App.tr("Signature:")
            visible: keying.handover !== ""
            readOnly: true
            selectByMouse: true
            text: keying.handover
            font.family: "monospace"
        }
    }

    FileDialog {
        id: publics

        currentFolder: "file://" + keying.where
        nameFilters: [App.tr("Public keys") + " (*.pub)"]
        onAccepted: newKey.text = App.path(selectedFile)
    }
}
