import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: preset

    required property var page
    readonly property var document: page.view.id === undefined ? ({}) : JSON.parse(page.editor.value("/preset") || "{}")
    readonly property var base: document.base ?? ({})
    readonly property var values: document.values ?? ({})
    readonly property var files: document.files ?? ({})
    readonly property var found: library.list.find(item => item.id === entry.editText.trim())

    function use() {
        const id = entry.editText.trim();
        const range = version.text.trim();
        if (id === "" || (range === "" && !found))
            return;
        page.manifestEdit("preset", {
                "base": {
                    "id": id,
                    "version": range !== "" ? range : found.version
                }
            });
    }

    function typed(text) {
        try {
            return JSON.parse(text);
        } catch (e) {
            return text;
        }
    }

    function setValue() {
        const values = {};
        values[name.text.trim()] = typed(value.text.trim());
        page.manifestEdit("preset", {
                "values": values
            });
        name.text = "";
        value.text = "";
    }

    function setFile(url) {
        const files = {};
        files[name.text.trim()] = App.path(url);
        page.manifestEdit("preset", {
                "files": files
            });
        name.text = "";
    }

    function drop(id) {
        page.manifestEdit("preset", {
                "clear": [id]
            });
    }

    PackageInstalled {
        id: library
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        textFormat: Text.PlainText
        text: (base.id ?? "") === "" ? App.tr("Choose the package this preset is for: it must be installed, or type its id and a version range.") : App.tr("Preset for %1, version %2", base.id, base.version)
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.ComboBox {
            id: entry

            Layout.fillWidth: true
            editable: true
            model: library.list.map(item => item.id)
            currentIndex: -1
            editText: preset.base.id ?? ""
            Accessible.name: App.tr("Base package")
        }

        QQC2.TextField {
            id: version

            Layout.preferredWidth: Kirigami.Units.gridUnit * 8
            placeholderText: preset.found ? App.tr("Compatible with %1", preset.found.version) : App.tr("^1.2")
            Accessible.name: App.tr("Version range")
            onAccepted: preset.use()
        }

        QQC2.Button {
            text: App.tr("Use as base")
            icon.name: "dialog-ok-apply"
            enabled: entry.editText.trim() !== "" && (!!preset.found || version.text.trim() !== "")
            onClicked: preset.use()
        }
    }

    Repeater {
        model: Object.keys(preset.values).concat(Object.keys(preset.files))

        delegate: RowLayout {
            id: row

            required property string modelData
            readonly property bool file: preset.files[modelData] !== undefined

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                textFormat: Text.PlainText
                text: row.modelData + ": " + (row.file ? preset.files[row.modelData] : JSON.stringify(preset.values[row.modelData]))
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: preset.drop(row.modelData)
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            id: name

            Layout.preferredWidth: Kirigami.Units.gridUnit * 8
            placeholderText: App.tr("Setting id")
            Accessible.name: App.tr("Setting id")
        }

        QQC2.TextField {
            id: value

            Layout.fillWidth: true
            placeholderText: App.tr("Value, such as #3d5aff or 0.4")
            Accessible.name: App.tr("Value")
            onAccepted: if (name.text.trim() !== "")
                preset.setValue()
        }

        QQC2.Button {
            text: App.tr("Set value")
            icon.name: "list-add"
            enabled: name.text.trim() !== "" && value.text.trim() !== ""
            onClicked: preset.setValue()
        }

        QQC2.Button {
            text: App.tr("Picture or video…")
            icon.name: "document-open"
            enabled: name.text.trim() !== ""
            onClicked: chooser.open()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("Values are the settings of the base package this preset sets; a picture or video is copied into the package. Title, license and the preview picture are in the package details.")
    }

    FileDialog {
        id: chooser

        nameFilters: [App.tr("Pictures and videos (*.png *.jpg *.jpeg *.webp *.gif *.webm *.mp4 *.mkv)")]
        onAccepted: preset.setFile(selectedFile)
    }
}
