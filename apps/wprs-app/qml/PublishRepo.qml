import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

ColumnLayout {
    id: repo

    required property var where
    readonly property var publish: where.publish
    readonly property var known: JSON.parse(publish.memory.repos || "[]")
    property string folder: known.length > 0 ? known[0].folder : ""
    property bool making: known.length === 0
    property var opened: ({})
    readonly property string slug: naming.text.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "repository"
    readonly property string target: making ? (place.text !== "" ? place.text : Publishing.parent(publish.editing ? publish.page.editor.folder : publish.file) + "/" + slug) : folder
    readonly property string name: making ? naming.text.trim() : (opened.name ?? (entry().name ?? Publishing.base(folder)))
    readonly property string key: (entry().key ?? "") !== "" ? entry().key : publish.sign.secret
    readonly property bool ready: making ? naming.text.trim() !== "" : folder !== "" && opened.error === undefined
    readonly property alias shelf: shelf

    function entry() {
        return known.find(r => r.folder === folder) ?? ({});
    }

    function look() {
        if (making || folder === "") {
            opened = {};
            return;
        }
        opened = JSON.parse(stalls.open(folder) || "{}");
        const expires = Date.parse(entry().expires ?? "");
        if (opened.root && opened.root.complete === false)
            publish.fault = {
                "kind": "raiz",
                "title": App.tr("“%1” is not signed yet", name),
                "text": App.tr("Without that signature nobody can install from it. It is signed once, with its key."),
                "details": App.tr("root: %1 of %2 required root signatures", (opened.root.signers ?? []).length, opened.root.threshold ?? 1)
            };
        else if (!isNaN(expires) && expires < Date.now())
            publish.fault = {
                "kind": "indice",
                "title": App.tr("The list of “%1” expired on %2", name, Qt.formatDate(new Date(expires), "dd-MM")),
                "text": App.tr("Whoever added it sees nothing new until you publish it again."),
                "details": App.tr("index expired %1", entry().expires)
            };
        else
            publish.fault = null;
    }

    function choose(path) {
        making = false;
        folder = path;
        look();
    }

    function remember(secret, expires) {
        const rest = known.filter(r => r.folder !== target);
        publish.memory.repos = JSON.stringify([{
                "folder": target,
                "name": name,
                "key": secret,
                "expires": expires
            }].concat(rest).slice(0, 6));
    }

    function address() {
        const fresh = JSON.parse(stalls.open(target) || "{}");
        return fresh.root ? (fresh.root.address ?? "") : "";
    }

    function signRoot() {
        shelf.root();
    }

    function republish() {
        shelf.again();
    }

    spacing: Look.e2

    Repeater {
        model: repo.known

        delegate: Opcion {
            required property var modelData

            Layout.fillWidth: true
            icon: !repo.making && repo.folder === modelData.folder ? "radio-button-fill" : "circle"
            lit: !repo.making && repo.folder === modelData.folder
            text: modelData.name
            note: App.tr("folder “%1”", Publishing.base(modelData.folder))
            onClicked: repo.choose(modelData.folder)
        }
    }

    Opcion {
        Layout.fillWidth: true
        icon: repo.making ? "radio-button-fill" : "circle"
        lit: repo.making
        text: App.tr("Create one here…")
        onClicked: {
            repo.making = true;
            repo.publish.fault = null;
        }
    }

    QQC2.TextField {
        id: naming

        Layout.fillWidth: true
        visible: repo.making
        placeholderText: App.tr("Name")
        Accessible.name: App.tr("Name")
    }

    RowLayout {
        Layout.fillWidth: true
        visible: repo.making

        QQC2.TextField {
            id: place

            Layout.fillWidth: true
            placeholderText: App.tr("Folder")
            Accessible.name: App.tr("Folder")
            text: ""
        }

        QQC2.Button {
            flat: true
            text: App.tr("Choose…")
            onClicked: parents.open()
        }
    }

    PublishNote {
        Layout.fillWidth: true
        visible: repo.making
        text: App.tr("Its keys are created, it is signed and its list is published; we tell you each step. For others to add it, the folder has to be somewhere they can reach.")
    }

    QQC2.Button {
        flat: true
        text: App.tr("Open another…")
        onClicked: others.open()
    }

    PublishShelf {
        id: shelf

        repo: repo
    }

    Stall {
        id: stalls
    }

    FolderDialog {
        id: others

        onAccepted: repo.choose(App.path(selectedFolder))
    }

    FolderDialog {
        id: parents

        onAccepted: place.text = App.path(selectedFolder) + "/" + repo.slug
    }
}
