import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property var repos: []
    property string error
    property string removing
    readonly property list<QtObject> tools: [
        Kirigami.Action {
            text: App.tr("Add repository…")
            icon.name: "list-add"
            onTriggered: adder.open()
        },
        Kirigami.Action {
            text: App.tr("Upload package…")
            icon.name: "document-export"
            onTriggered: packages.open()
        },
        Kirigami.Action {
            text: App.tr("Refresh all")
            icon.name: "view-refresh"
            enabled: page.repos.length > 0
            onTriggered: App.act("Refresh", [""], "Repos1")
        }
    ]

    function load() {
        App.query("List", [], "Repos1", reply => {
                if (!page)
                    return;
                page.repos = reply.repos;
                page.error = "";
            }, error => page && (page.error = error.message));
    }

    function exercise() {
        installer.watching = [7];
        installer.announce([{
                    "id": 7,
                    "state": "done",
                    "target": "0123abcd/org.example.dusk@1.1.0"
                }]);
        if (installer.message === "" || !installer.message.includes("org.example.dusk") || !installer.message.includes("1.1.0") || installer.watching.length !== 0)
            throw new Error("an install that finished was not confirmed: " + installer.message);
        installer.message = "";
        const shown = Qt.createComponent("RepoAddress.qml").createObject(page, {
                "text": "file:///repo/?fingerprint=ab&repo_id=cd"
            });
        shown.copy();
        const board = Qt.createQmlObject("import QtQuick; TextEdit { visible: false }", page);
        board.paste();
        if (!shown.copied || board.text !== "file:///repo/?fingerprint=ab&repo_id=cd")
            throw new Error("the repository address was not copied: " + board.text);
        board.destroy();
        shown.destroy();
        tabs.currentIndex = 3;
        tabs.currentIndex = 1;
    }

    title: App.tr("Repositories")
    Component.onCompleted: load()
    actions: uploader.shown ? uploader.actions : tools
    header: QQC2.TabBar {
        id: tabs

        QQC2.TabButton {
            text: App.tr("Repositories")
        }

        QQC2.TabButton {
            text: App.tr("Browse")
        }

        QQC2.TabButton {
            text: App.tr("Installed")
        }

        QQC2.TabButton {
            text: App.tr("Static repository")
        }
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "repos")
                page.load();
        }
    }

    Kirigami.InlineMessage {
        id: confirmation

        anchors.left: parent.left
        anchors.right: parent.right
        z: 1
        type: Kirigami.MessageType.Positive
        text: installer.message
        visible: text !== ""
        showCloseButton: true
        onVisibleChanged: if (!visible)
            installer.message = ""
    }

    StackLayout {
        anchors.fill: parent
        anchors.topMargin: confirmation.visible ? confirmation.height + Kirigami.Units.smallSpacing : 0
        currentIndex: tabs.currentIndex

        RepoList {
            repos: page.repos
            error: page.error
            onRemove: repo => {
                page.removing = repo;
                remover.open();
            }
        }

        Browse {
            installer: installer
        }

        Installed {
            installer: installer
        }

        RepoStatic {
        }
    }

    Installer {
        id: installer
    }

    FileDialog {
        id: packages

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: uploader.openWith(App.path(selectedFile))
    }

    Publish {
        id: uploader
    }

    Kirigami.Dialog {
        id: adder

        title: App.tr("Add repository")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("Add", [url.text.trim()], "Repos1")

        QQC2.TextField {
            id: url

            implicitWidth: Kirigami.Units.gridUnit * 24
            placeholderText: "https://example.org/repo/?fingerprint=…"
        }
    }

    Kirigami.Dialog {
        id: remover

        title: App.tr("Remove repository")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("Remove", [page.removing, uninstall.checked], "Repos1")

        QQC2.CheckBox {
            id: uninstall

            text: App.tr("Also uninstall its packages")
        }
    }
}
