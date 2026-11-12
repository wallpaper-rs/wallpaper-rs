import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    readonly property int blocking: (page.view.problems ?? []).filter(problem => !problem.warning).length
    readonly property string why: page.editor.busy ? App.tr("Wait: the editor is still working.") : page.editor.dirty ? App.tr("You have changes that are not saved yet. Save them to build or apply.") : blocking > 0 ? App.tr("The scene has %1 problems to solve first; they are listed in the Problems panel.", blocking) : ""
    readonly property bool ready: !page.editor.dirty && !page.editor.busy && !(page.view.problems ?? []).some(problem => !problem.warning)

    title: App.tr("Build and apply")
    padding: Kirigami.Units.largeSpacing
    preferredWidth: Kirigami.Units.gridUnit * 28
    standardButtons: Kirigami.Dialog.Close

    ColumnLayout {
        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: !dialog.ready
            text: dialog.why
        }

        QQC2.Button {
            visible: dialog.page.editor.dirty && !dialog.page.editor.busy
            text: App.tr("Save now")
            icon.name: "document-save"
            onClicked: dialog.page.editor.save()
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: out

                Kirigami.FormData.label: App.tr("Package file:")
                placeholderText: App.tr("Next to the folder, named after the id and version")
            }

            RowLayout {
                Kirigami.FormData.label: App.tr("Author key:")

                QQC2.TextField {
                    id: key

                    Layout.fillWidth: true
                    placeholderText: App.tr("None: the package is not signed")
                }

                QQC2.Button {
                    icon.name: "document-open"
                    text: App.tr("Choose…")
                    onClicked: keys.open()
                }

                QQC2.Button {
                    icon.name: "dialog-password"
                    text: App.tr("Keys…")
                    onClicked: keyring.open()
                }
            }

            Kirigami.PasswordField {
                id: password

                Kirigami.FormData.label: App.tr("Key password:")
                enabled: key.text !== ""
            }

            QQC2.Button {
                Kirigami.FormData.label: App.tr("Before building:")
                text: App.tr("Review the package…")
                icon.name: "view-list-details"
                onClicked: reviewing.open()
            }

            QQC2.Button {
                Kirigami.FormData.label: App.tr("Publish:")
                text: App.tr("Add to a repository…")
                icon.name: "document-send"
                onClicked: shelving.open()
            }

            QQC2.Button {
                text: App.tr("Build the package")
                icon.name: "run-build"
                enabled: dialog.ready
                onClicked: {
                    dialog.page.editor.build(out.text, key.text, password.text);
                    password.text = "";
                }
            }

            QQC2.ComboBox {
                id: target

                Kirigami.FormData.label: App.tr("Screen:")
                model: [App.tr("Every screen")].concat(App.outputs)
            }

            QQC2.Button {
                text: App.tr("Apply to the screen")
                icon.name: "preferences-desktop-wallpaper"
                enabled: dialog.ready
                onClicked: dialog.page.editor.apply(target.currentIndex > 0 ? App.outputs[target.currentIndex - 1] : "")
            }
        }
    }

    SceneReview {
        id: reviewing

        page: dialog.page
    }

    SceneShelve {
        id: shelving

        page: dialog.page
    }

    SceneKeys {
        id: keyring

        onPicked: secret => key.text = secret
    }

    FileDialog {
        id: keys

        nameFilters: [App.tr("Secret keys") + " (*.key)"]
        onAccepted: key.text = App.path(selectedFile)
    }
}
