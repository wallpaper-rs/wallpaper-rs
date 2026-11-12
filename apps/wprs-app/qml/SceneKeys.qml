import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: keying

    property var found: []
    property string message: ""
    property bool failed: false
    property string made: ""
    property string handover: ""
    property string chosen: ""
    readonly property alias backend: backend
    readonly property string where: backend.folder()

    signal picked(string secret)

    function refresh() {
        found = JSON.parse(backend.keys(where) || "[]");
    }

    function create(replace) {
        backend.create(where, name.text.trim(), password.text, replace);
        password.text = "";
    }

    function make() {
        if (backend.exists(where, name.text.trim()))
            replacing.open();
        else
            create(false);
    }

    function take(operation, answer) {
        const reply = JSON.parse(answer);
        failed = reply.error !== undefined;
        if (failed) {
            message = reply.error;
            return;
        }
        if (operation === "create") {
            made = reply.secret;
            message = reply.replaced ? App.tr("Replaced the key pair %1; its fingerprint is %2", reply.name, reply.fingerprint) : App.tr("Created the key pair %1; its fingerprint is %2", reply.name, reply.fingerprint);
            refresh();
        } else {
            handover = reply.signature;
            message = App.tr("Signed the hand-over from %1 to %2", reply.from, reply.to);
        }
    }

    title: App.tr("Author keys")
    padding: Kirigami.Units.largeSpacing
    preferredWidth: Kirigami.Units.gridUnit * 34
    standardButtons: Kirigami.Dialog.Close
    onAboutToShow: refresh()

    Signing {
        id: backend

        onAnswered: (operation, answer) => keying.take(operation, answer)
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Secret keys stay in %1, readable only by you. Nothing here ever shows one; keep a copy somewhere safe, because a lost key cannot be recovered.", keying.where)
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: keying.message !== ""
            type: keying.failed ? Kirigami.MessageType.Error : Kirigami.MessageType.Positive
            text: keying.message
        }

        Repeater {
            model: keying.found

            delegate: RowLayout {
                required property var modelData

                Layout.fillWidth: true

                QQC2.RadioButton {
                    Layout.fillWidth: true
                    checked: keying.chosen === modelData.secret
                    text: modelData.name + (modelData.encrypted ? " · " + App.tr("password") : "") + (modelData.fingerprint ? " · " + modelData.fingerprint.slice(0, 16) : "")
                    onClicked: keying.chosen = modelData.secret
                }

                QQC2.Button {
                    text: App.tr("Use to sign")
                    icon.name: "document-sign"
                    onClicked: {
                        keying.picked(modelData.secret);
                        keying.close();
                    }
                }
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: keying.found.length === 0
            wrapMode: Text.Wrap
            text: App.tr("There are no keys yet.")
        }

        Kirigami.Heading {
            level: 4
            text: App.tr("New key pair")
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: name

                Kirigami.FormData.label: App.tr("Name:")
                placeholderText: App.tr("author")
                maximumLength: 64
            }

            Kirigami.PasswordField {
                id: password

                Kirigami.FormData.label: App.tr("Password:")
                placeholderText: App.tr("Empty for a key without one")
            }

            QQC2.Button {
                text: App.tr("Create the key pair")
                icon.name: "dialog-password"
                enabled: name.text.trim() !== "" && !backend.busy
                onClicked: keying.make()
            }
        }

        SceneHandover {
            Layout.fillWidth: true
            keying: keying
        }
    }

    Kirigami.PromptDialog {
        id: replacing

        title: App.tr("Replace the key pair?")
        subtitle: App.tr("A key pair named %1 already exists. Replacing it destroys its secret key for good: packages signed with it can no longer be followed by new versions you sign.", name.text.trim())
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: keying.create(true)
    }
}
