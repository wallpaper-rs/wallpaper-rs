import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: copying

    property string file: ""
    property var facts: ({})
    property string problem: ""
    readonly property bool refused: facts.verdict === "refused"
    readonly property bool ready: facts.id !== undefined && folder.text !== "" && newId.text !== "" && (!refused || mine.checked)
    readonly property alias helper: helper

    function start(path) {
        file = path;
        problem = "";
        mine.checked = false;
        facts = JSON.parse(helper.copyFacts(path));
        if (facts.error !== undefined) {
            problem = facts.error;
            facts = ({});
        } else {
            folder.text = helper.folderFor(facts.title);
            newId.text = facts.new_id;
        }
        open();
    }

    function make() {
        const reply = JSON.parse(helper.copyInto(file, folder.text, newId.text, mine.checked));
        if (reply.error !== undefined) {
            problem = reply.error;
            return;
        }
        copying.close();
        App.compose(reply.folder, "", "");
    }

    title: App.tr("Edit a copy")
    padding: Kirigami.Units.largeSpacing
    preferredWidth: Kirigami.Units.gridUnit * 30
    standardButtons: Kirigami.Dialog.Close

    SceneEditor {
        id: helper
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("The copy is unpacked into a folder of its own with a new id, so the installed package never changes. Its license and credits stay, and it credits the original as the work it comes from.")
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: copying.problem !== ""
            type: Kirigami.MessageType.Error
            text: copying.problem
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: copying.refused
            type: Kirigami.MessageType.Warning
            text: App.tr("The license %1 allows no derived works. Edit a copy only if you made this package or its author allows it.", copying.facts.license ?? "")
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: !copying.refused && (copying.facts.conditions ?? []).length > 0
            type: Kirigami.MessageType.Information
            text: App.tr("The license %1 asks for: %2.", copying.facts.license ?? "", (copying.facts.conditions ?? []).join(", "))
        }

        QQC2.CheckBox {
            id: mine

            visible: copying.refused
            text: App.tr("I made this package or its author allows a copy")
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.Label {
                Kirigami.FormData.label: App.tr("Package:")
                text: (copying.facts.title ?? "") + " · " + (copying.facts.kind ?? "") + " " + (copying.facts.version ?? "")
            }

            RowLayout {
                Kirigami.FormData.label: App.tr("Folder:")

                QQC2.TextField {
                    id: folder

                    Layout.fillWidth: true
                }

                QQC2.Button {
                    icon.name: "document-open-folder"
                    text: App.tr("Choose…")
                    onClicked: folders.open()
                }
            }

            QQC2.TextField {
                id: newId

                Kirigami.FormData.label: App.tr("Id of the copy:")
            }

            QQC2.Button {
                text: App.tr("Edit a copy")
                icon.name: "edit-copy"
                enabled: copying.ready
                onClicked: copying.make()
            }
        }
    }

    FolderDialog {
        id: folders

        onAccepted: folder.text = App.path(selectedFolder)
    }
}
