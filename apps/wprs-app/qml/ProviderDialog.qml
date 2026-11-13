import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    readonly property var kinds: App.schema.providers ?? []
    readonly property var kind: kinds[chooser.currentIndex] ?? {}
    readonly property var fields: kind.options ?? []
    readonly property string address: values.url ?? values.server ?? ""
    readonly property bool plain: /^http:\/\/(?!localhost|127\.|\[::1\])/i.test(address)
    readonly property bool noKeyring: error !== "" && errorName.endsWith("KeyringUnavailable")
    property var filer: null
    property string errorName
    property var values: ({})
    property string error

    title: App.tr("Add provider")
    standardButtons: Kirigami.Dialog.Cancel
    customFooterActions: [
        Kirigami.Action {
            text: App.tr("Add")
            icon.name: "list-add"
            onTriggered: {
                dialog.errorName = "";
                App.addProvider(name.text, dialog.kind.kind, dialog.values, secret.text, () => dialog.close(), error => {
                    dialog.errorName = error.name;
                    dialog.error = error.message;
                });
                secret.clear();
            }
        }
    ]
    onOpened: {
        values = {};
        error = "";
        errorName = "";
    }
    onClosed: secret.clear()

    Kirigami.FormLayout {
        Kirigami.InlineMessage {
            Layout.fillWidth: true
            type: Kirigami.MessageType.Error
            visible: dialog.error !== ""
            text: dialog.noKeyring ? App.tr("No system keyring answers, so the secret cannot be stored. Start or unlock your keyring, or keep secrets in a private file.") : dialog.error
            actions: [
                Kirigami.Action {
                    text: App.tr("Keep secrets in a private file…")
                    icon.name: "document-encrypt"
                    visible: dialog.noKeyring && dialog.filer !== null
                    onTriggered: dialog.filer.open()
                }
            ]
        }

        QQC2.TextField {
            id: name

            Kirigami.FormData.label: App.tr("Name:")
            placeholderText: App.tr("lowercase letters, digits, - and _")
            validator: RegularExpressionValidator {
                regularExpression: /[a-z][a-z0-9_-]{0,31}/
            }
        }

        QQC2.ComboBox {
            id: chooser

            Kirigami.FormData.label: App.tr("Kind:")
            model: dialog.kinds.map(kind => kind.kind)
            onActivated: dialog.values = {}
        }

        Settings {
            daemon: App
            model: dialog.fields
            values: dialog.values
            onEdited: (key, value) => dialog.values = Object.assign({}, dialog.values, {
                        [key]: value
                    })
        }

        Kirigami.PasswordField {
            id: secret

            Kirigami.FormData.label: Labels.field(dialog.kind.secret?.name ?? "password") + ":"
            visible: !!dialog.kind.secret
            placeholderText: dialog.kind.secret?.required ? App.tr("Required") : App.tr("Optional")
        }

        QQC2.Label {
            visible: dialog.plain
            Layout.maximumWidth: Kirigami.Units.gridUnit * 18
            text: App.tr("This address uses plain http: the password or key, and the pictures, travel over the network without encryption.")
            color: Kirigami.Theme.negativeTextColor
            wrapMode: Text.Wrap
        }
    }
}
