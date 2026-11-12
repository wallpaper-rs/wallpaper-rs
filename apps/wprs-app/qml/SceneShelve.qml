import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: shelving

    required property var page
    property string folder: ""
    property var shown: null
    property string message: ""
    property bool failed: false
    readonly property alias backend: backend
    readonly property alias publishing: publishing
    readonly property string built: String(page.notice ?? "").split("\n")[0]
    property string id: ""
    readonly property alias lineage: lineage
    readonly property bool open_: shown !== null

    function take(answer) {
        const reply = JSON.parse(answer);
        failed = reply.error !== undefined;
        if (failed) {
            message = reply.error;
            return;
        }
        shown = reply;
        if (reply.published)
            message = App.tr("Published index %1 with %2 packages; it expires %3", reply.published.version, reply.published.packages, reply.published.expires);
        else if (reply.added)
            message = App.tr("Added %1. Feature it or publish the repository next.", reply.added.join(", "));
        else
            message = reply.changed ? App.tr("Changed. Publish the repository next.") : "";
    }

    function chosen(path) {
        folder = path;
        take(backend.open(path));
    }

    function add() {
        backend.add(folder, JSON.stringify([file.text]), unlisted.checked);
    }

    title: App.tr("Add to a static repository")
    padding: Kirigami.Units.largeSpacing
    preferredWidth: Kirigami.Units.gridUnit * 32
    standardButtons: Kirigami.Dialog.Close
    onAboutToShow: {
        id = JSON.parse(page.editor.value("/manifest/id") || '""');
        file.text = built.endsWith(".wpak") ? built : "";
    }

    Stall {
        id: backend

        onAnswered: answer => shelving.take(answer)
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Add the package you built to a static repository folder, optionally unlisted, feature it with a note and publish the repository with its online key. New repositories and their keys are made in Repositories.")
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: shelving.message !== ""
            type: shelving.failed ? Kirigami.MessageType.Error : Kirigami.MessageType.Positive
            text: shelving.message
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                text: shelving.shown ? shelving.shown.name + " · " + shelving.shown.repo_id : App.tr("No repository chosen")
            }

            QQC2.Button {
                icon.name: "document-open-folder"
                text: App.tr("Open a static repository…")
                enabled: !backend.busy
                onClicked: folders.open()
            }
        }

        RepoAddress {
            Layout.fillWidth: true
            text: shelving.shown?.root?.address ?? ""
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true
            enabled: shelving.open_ && !backend.busy

            RowLayout {
                Kirigami.FormData.label: App.tr("Package file:")

                QQC2.TextField {
                    id: file

                    Layout.fillWidth: true
                    placeholderText: App.tr("Build the package first")
                    onTextChanged: lineage.inspect(text)
                }

                QQC2.Button {
                    icon.name: "document-open"
                    text: App.tr("Choose…")
                    onClicked: wpaks.open()
                }
            }

            QQC2.CheckBox {
                id: unlisted

                text: App.tr("Add unlisted")
                QQC2.ToolTip.text: App.tr("Out of browsing and search; installing by id still works")
                QQC2.ToolTip.visible: hovered
            }

            SceneLineage {
                id: lineage

                Layout.fillWidth: true
            }

            QQC2.Button {
                text: App.tr("Add to the repository")
                icon.name: "list-add"
                enabled: file.text !== "" && lineage.confirmed
                onClicked: shelving.add()
            }

            QQC2.TextField {
                id: note

                Kirigami.FormData.label: App.tr("Note when featured:")
                maximumLength: 140
            }

            RowLayout {
                Kirigami.FormData.label: App.tr("Featured:")

                QQC2.Button {
                    text: App.tr("Feature")
                    icon.name: "favorite"
                    onClicked: backend.feature(shelving.folder, shelving.id, note.text, true)
                }

                QQC2.Button {
                    text: App.tr("Stop featuring")
                    icon.name: "emblem-unfavorite"
                    onClicked: backend.feature(shelving.folder, shelving.id, "", false)
                }
            }

            QQC2.Button {
                text: App.tr("Publish…")
                icon.name: "document-send"
                onClicked: publishing.open()
            }
        }
    }

    RepoPublish {
        id: publishing

        onPublish: (key, password, days) => backend.publish(shelving.folder, key, password, days)
    }

    FolderDialog {
        id: folders

        onAccepted: shelving.chosen(App.path(selectedFolder))
    }

    FileDialog {
        id: wpaks

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: file.text = App.path(selectedFile)
    }
}
