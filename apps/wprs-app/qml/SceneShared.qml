import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string file: ""
    property var info: null
    property var chosen: []
    property var installed: []
    property string uid: ""
    readonly property alias paths: typed
    readonly property alias listed: found

    function pick(path) {
        uid = "";
        file = path;
        info = JSON.parse(shelf.parts(path) || "null");
        chosen = (info?.parts ?? []).map(p => p.key);
    }

    function choose(uid) {
        App.query("GetItem", [uid], "Library1", reply => {
            if (!dialog || !reply?.item?.path)
                return;
            dialog.pick(reply.item.path);
            dialog.uid = uid;
        }, () => {});
    }

    function toggle(key, on) {
        const rest = chosen.filter(k => k !== key);
        chosen = on ? rest.concat([key]) : rest;
    }

    title: App.tr("Import from an assets package")
    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
    onOpened: App.query("Search", ["kind:assets", 0, 200], "Library1", reply => dialog && (dialog.installed = (reply?.items ?? []).filter(item => !item.missing)), () => {})
    onAccepted: {
        if (!file || info?.error || chosen.length === 0)
            return;
        page.run("import_assets", {
            "package": file,
            "parts": chosen
        });
    }

    Shelf {
        id: shelf
    }

    ColumnLayout {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 28

        Kirigami.Heading {
            level: 4
            visible: dialog.installed.length > 0
            text: App.tr("Installed assets packages")
        }

        SceneRepeater {
            id: found

            rows: dialog.installed

            delegate: QQC2.ItemDelegate {
                required property int index
                readonly property var modelData: found.rows[index]

                Layout.fillWidth: true
                focusPolicy: Qt.StrongFocus
                text: modelData.title
                icon.name: "package-x-generic"
                highlighted: dialog.uid === modelData.uid
                onClicked: dialog.choose(modelData.uid)
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: dialog.installed.length === 0
            text: App.tr("No assets packages are installed in the library yet. Pick a package file or type its path.")
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: typed

                Layout.fillWidth: true
                placeholderText: App.tr("Path of an assets package")
                Accessible.name: App.tr("Path of an assets package")
                onAccepted: dialog.pick(text.trim())
            }

            QQC2.Button {
                text: App.tr("Use path")
                enabled: typed.text.trim() !== ""
                onClicked: dialog.pick(typed.text.trim())
            }

            QQC2.Button {
                icon.name: "document-open"
                text: dialog.file ? dialog.file.split("/").pop() : App.tr("Pick an assets package")
                onClicked: files.open()
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: dialog.info?.error !== undefined
            color: Look.coral
            text: dialog.info?.error ?? ""
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: (dialog.info?.parts ?? []).length > 0
            text: App.tr("%1 %2 · %3 · by %4", dialog.info?.title ?? "", dialog.info?.version ?? "", dialog.info?.license ?? "", (dialog.info?.authors ?? []).join(", "))
        }

        GridLayout {
            Layout.fillWidth: true
            columns: 2

            SceneRepeater {
                id: listed

                rows: dialog.info?.parts ?? []

                delegate: SceneSharedPart {
                    required property int index
                    readonly property var modelData: listed.rows[index]

                    part: modelData
                    chosen: dialog.chosen.includes(modelData.key)
                    onToggled: on => dialog.toggle(modelData.key, on)
                }
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            font: Kirigami.Theme.smallFont
            text: App.tr("The chosen parts and their files are copied into this scene, so it works without the package; their credits and the origin are written for you.")
        }
    }

    FileDialog {
        id: files

        nameFilters: [App.tr("Assets packages") + " (*.wpak)"]
        onAccepted: dialog.pick(App.path(selectedFile))
    }
}
