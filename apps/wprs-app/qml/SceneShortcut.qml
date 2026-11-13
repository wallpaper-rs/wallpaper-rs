import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: shortcut

    required property var daemon
    required property var modelData
    readonly property var value: modelData.value ?? null
    readonly property string chosen: value ? Object.keys(value)[0] ?? "" : ""
    readonly property string target: value ? String(value[chosen] ?? "") : ""
    readonly property var kinds: [["uri", daemon.tr("Web address")], ["file", daemon.tr("File")], ["folder", daemon.tr("Folder")], ["app", daemon.tr("Application")]]

    property var applications: ({
            "available": true,
            "applications": []
        })
    property string problem: ""
    readonly property var apps: applications.applications ?? []
    readonly property string wanted: kind.currentIndex === 0 || kind.currentIndex === 3 ? typed.text.trim() : ""

    signal edited(string key, var value)

    function candidate() {
        const value = {};
        value[kinds[kind.currentIndex][0]] = typed.text.trim();
        return JSON.stringify(value);
    }

    function verify() {
        const asked = wanted;
        if (asked === "") {
            problem = "";
            return;
        }
        daemon.query("CheckShortcut", [candidate()], "Daemon1", () => shortcut && shortcut.wanted === asked && (shortcut.problem = ""), error => shortcut && shortcut.wanted === asked && (shortcut.problem = error.message ?? ""));
    }

    function listApplications() {
        daemon.query("ListApplications", [], "Daemon1", reply => shortcut && (shortcut.applications = reply));
    }

    function pick(kind, text) {
        const value = {};
        value[kind] = text;
        edited(modelData.key, value);
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: shortcut.value ? shortcut.kinds.find(k => k[0] === shortcut.chosen)?.[1] + ": " + shortcut.target : shortcut.daemon.tr("Nothing is chosen; a click on the wallpaper opens nothing.")
        textFormat: Text.PlainText
        wrapMode: Text.WrapAnywhere
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.ComboBox {
            id: kind

            model: shortcut.kinds.map(k => k[1])
            currentIndex: Math.max(0, shortcut.kinds.findIndex(k => k[0] === shortcut.chosen))
            Accessible.name: shortcut.daemon.tr("What it opens")
            onCurrentIndexChanged: {
                shortcut.problem = "";
                check.restart();
                if (currentIndex === 3 && shortcut.apps.length === 0)
                    shortcut.listApplications();
            }
        }

        QQC2.TextField {
            id: typed

            Layout.fillWidth: true
            visible: kind.currentIndex === 0 || kind.currentIndex === 3
            placeholderText: kind.currentIndex === 0 ? "https://" : shortcut.daemon.tr("Application id, such as org.kde.dolphin")
            text: shortcut.chosen === shortcut.kinds[kind.currentIndex][0] ? shortcut.target : ""
            onTextChanged: check.restart()
            onAccepted: if (shortcut.problem === "")
                shortcut.pick(shortcut.kinds[kind.currentIndex][0], text.trim())
        }

        QQC2.Button {
            enabled: shortcut.problem === "" && (kind.currentIndex === 1 || kind.currentIndex === 2 || typed.text.trim() !== "")
            text: kind.currentIndex === 1 || kind.currentIndex === 2 ? shortcut.daemon.tr("Choose…") : shortcut.daemon.tr("Use")
            icon.name: kind.currentIndex === 2 ? "folder-open" : kind.currentIndex === 1 ? "document-open" : "dialog-ok-apply"
            onClicked: kind.currentIndex === 1 ? files.open() : kind.currentIndex === 2 ? folders.open() : typed.accepted()
        }

        QQC2.ToolButton {
            visible: shortcut.value !== null
            icon.name: "edit-clear"
            text: shortcut.daemon.tr("Clear")
            display: QQC2.AbstractButton.IconOnly
            onClicked: shortcut.edited(shortcut.modelData.key, null)
        }
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        visible: kind.currentIndex === 3 && shortcut.applications.available !== false
        model: shortcut.apps.map(a => a.name === a.id ? a.id : a.name + " (" + a.id + ")")
        currentIndex: shortcut.apps.findIndex(a => a.id === typed.text.trim())
        displayText: currentIndex >= 0 ? currentText : shortcut.daemon.tr("Choose an installed application…")
        Accessible.name: shortcut.daemon.tr("Installed applications")
        onActivated: index => typed.text = shortcut.apps[index].id
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: shortcut.problem !== ""
        text: shortcut.problem
        textFormat: Text.PlainText
        wrapMode: Text.Wrap
        color: Look.coral
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: kind.currentIndex === 3 && shortcut.applications.available === false
        text: shortcut.daemon.tr("Applications open only outside the Flatpak.")
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
    }

    Timer {
        id: check

        interval: 300
        onTriggered: shortcut.verify()
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: shortcut.daemon.tr("Only you choose what it opens: the wallpaper never sees it and asks for it only when you click it on a screen that takes clicks. Web addresses open through the desktop, which may ask first; applications open only outside the Flatpak.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        color: Kirigami.Theme.disabledTextColor
    }

    FileDialog {
        id: files

        onAccepted: shortcut.pick("file", shortcut.daemon.path(selectedFile))
    }

    FolderDialog {
        id: folders

        onAccepted: shortcut.pick("folder", shortcut.daemon.path(selectedFolder))
    }
}
