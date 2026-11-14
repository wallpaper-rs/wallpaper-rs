import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: keys

    required property var account
    property var items: []
    property string problem: ""
    property string pending: ""

    function load() {
        App.query("ListAuthorKeys", [], "Sync1", reply => keys && (keys.items = reply.items ?? []), () => keys && (keys.items = []));
    }

    function failed(error) {
        if (keys)
            keys.problem = error.message;
    }

    function add(text) {
        problem = "";
        App.act("AddAuthorKey", [text], "Sync1", () => keys && keys.load(), keys.failed);
    }

    function remove(fingerprint) {
        problem = "";
        App.act("RemoveAuthorKey", [fingerprint], "Sync1", () => keys && keys.load(), keys.failed);
    }

    function rotate(from, text, signature) {
        problem = "";
        App.act("RotateAuthorKey", [from, text, signature], "Sync1", () => keys && keys.load(), keys.failed);
    }

    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (topic === "account")
                keys.load();
        }
    }

    Seccion {
        text: App.tr("Author keys")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("The public keys with which you sign the packages you upload. Create a pair in the build dialog of the editor; only the public key goes here.")
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: keys.problem !== ""
        type: Kirigami.MessageType.Error
        text: keys.problem
    }

    Repeater {
        model: keys.items

        delegate: RowLayout {
            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                text: modelData.fingerprint
            }

            QQC2.Button {
                id: handOver

                text: App.tr("Hand over…")
                icon.name: "go-next"
                onClicked: {
                    keys.pending = modelData.fingerprint;
                    handing.opener = handOver;
                    handing.open();
                }
            }

            QQC2.Button {
                text: App.tr("Remove")
                icon.name: "edit-delete"
                onClicked: keys.remove(modelData.fingerprint)
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            id: added

            Layout.fillWidth: true
            placeholderText: App.tr("Public key file or text")
        }

        QQC2.Button {
            icon.name: "document-open"
            text: App.tr("Choose…")
            onClicked: files.open()
        }

        QQC2.Button {
            text: App.tr("Register the key")
            icon.name: "list-add"
            enabled: added.text.trim() !== ""
            onClicked: {
                keys.add(added.text.trim());
                added.text = "";
            }
        }
    }

    PanelChico {
        id: handing

        title: App.tr("Hand over to a new key")
        text: App.tr("Sign the hand-over with the old key in Manage keys… of the step “Sign it” when you publish, and paste the signature here.")
        quiet: App.tr("Cancel")
        principal: App.tr("OK")
        onAccepted: keys.rotate(keys.pending, successor.text.trim(), signature.text.trim())
        onClosed: {
            successor.text = "";
            signature.text = "";
        }

        QQC2.Label {
            width: handing.width - 2 * Look.e4
            text: App.tr("New public key:")
        }

        QQC2.TextField {
            id: successor

            width: handing.width - 2 * Look.e4
            Accessible.name: App.tr("New public key:")
        }

        QQC2.Label {
            width: handing.width - 2 * Look.e4
            text: App.tr("Signature:")
        }

        Area {
            id: signature

            width: handing.width - 2 * Look.e4
            font.family: "monospace"
            Accessible.name: App.tr("Signature:")
        }
    }

    FileDialog {
        id: files

        nameFilters: [App.tr("Public keys") + " (*.pub)"]
        onAccepted: added.text = App.path(selectedFile)
    }
}
