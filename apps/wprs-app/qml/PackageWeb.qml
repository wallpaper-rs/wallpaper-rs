import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.FormLayout {
    id: web

    required property var page
    readonly property var settings: page.manifest.web ?? ({})
    readonly property var listed: (page.view.files ?? []).filter(file => file.kind === "script").map(file => file.path)
    readonly property int most: 5120

    function put(values, clear, gesture) {
        page.manifestEdit("web", {
                "values": values,
                "clear": clear ?? []
            }, gesture === true);
    }

    function use(path) {
        put({
                "preload": path
            });
    }

    QQC2.ComboBox {
        id: preload

        Kirigami.FormData.label: App.tr("Script before the page:")
        model: [App.tr("None")].concat(web.listed)
        currentIndex: Math.max(0, web.listed.indexOf(web.settings.preload ?? "") + 1)
        onActivated: index => index === 0 ? web.put({}, ["preload"]) : web.use(web.listed[index - 1])
    }

    QQC2.Button {
        text: App.tr("Add a script…")
        icon.name: "list-add"
        onClicked: picker.open()
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Color while it loads:")

        SceneColor {
            id: background

            hex: web.settings.background ?? "#000000"
            alpha: false
            onPicked: hex => web.put({
                        "background": hex
                    })
        }

        QQC2.Button {
            text: App.tr("Default")
            icon.name: "edit-undo"
            enabled: web.settings.background !== undefined
            onClicked: web.put({}, ["background"])
        }
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Kept data:")

        SceneSlider {
            id: storage

            Layout.preferredWidth: Kirigami.Units.gridUnit * 12
            editor: web.page.editor
            from: 0
            to: web.most
            stepSize: 64
            value: web.settings.storage_kib ?? 0
            onMoved: web.put({
                    "storage_kib": Math.round(value)
                }, [], true)
        }

        QQC2.Label {
            text: storage.value === 0 ? App.tr("Nothing is kept") : App.tr("%1 KiB", Math.round(storage.value))
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("The page keeps its local storage, IndexedDB and cache up to this size, from 0 to 5120 KiB; 0 keeps nothing.")
    }

    FileDialog {
        id: picker

        fileMode: FileDialog.OpenFile
        nameFilters: [App.tr("JavaScript files") + " (*.js *.mjs)"]
        onAccepted: {
            const file = App.path(selectedFile);
            web.page.manifestEdit("add_file", {
                    "file": file
                });
            web.use(file.split("/").pop());
        }
    }
}
