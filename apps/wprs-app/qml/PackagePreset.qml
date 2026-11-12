import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: preset

    required property var page
    readonly property var document: page.view.id === undefined ? ({}) : (JSON.parse(page.editor.value("/preset") || "null") ?? ({}))
    readonly property var base: document.base ?? ({})
    readonly property var values: document.values ?? ({})
    readonly property var files: document.files ?? ({})
    readonly property var found: library.list.find(item => item.id === entry.editText.trim())
    readonly property var installedBase: library.list.find(item => item.id === (base.id ?? ""))
    property string basePath
    property var catalog: null

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

    function resolve() {
        basePath = "";
        if (!installedBase)
            return;
        App.query("GetItem", [installedBase.uid], "Library1", reply => preset && (preset.basePath = String(reply?.path ?? "")), () => {});
    }

    function load() {
        catalog = basePath ? JSON.parse(page.editor.rig("", JSON.stringify({
                        "base": {
                            "file": basePath
                        }
                    })) || "null") : null;
    }

    onInstalledBaseChanged: resolve()
    onBasePathChanged: load()

    Connections {
        function onViewChanged() {
            preset.load();
        }

        target: preset.page.editor
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

    QQC2.Label {
        Layout.fillWidth: true
        visible: (preset.base.id ?? "") !== "" && !preset.catalog
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: preset.installedBase ? App.tr("Reading the settings of the base package…") : App.tr("Install the base package to choose the settings this preset sets.")
    }

    PresetSettings {
        Layout.fillWidth: true
        visible: !!preset.catalog
        page: preset.page
        catalog: preset.catalog
    }

    PackagePresetPublish {
        Layout.fillWidth: true
        page: preset.page
        basePath: preset.basePath
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("Each setting of the base package has its own control; the ones in bold are set by this preset, and a picture or video is copied into the package.")
    }
}
