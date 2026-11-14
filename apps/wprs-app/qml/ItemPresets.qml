import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: presets

    required property var page
    readonly property var host: page.host
    property var global: []
    readonly property var all: (host.details.presets ?? []).concat(global)
    property var doomed: null

    function load() {
        App.query("ListPresets", [""], "Profiles1", reply => presets && (presets.global = reply.presets ?? []));
    }

    function caption(preset) {
        const value = preset.value ?? {};
        return [value.fit ? Labels.value(value.fit) : "", value.background ? App.tr("fill %1", Labels.colour(value.background)) : ""].filter(t => t).join(" · ");
    }

    function use(preset) {
        const output = App.outputs.length > 1 ? page.target : "";
        App.act("Apply", [host.uid, output, "", preset.uid], "Library1", () => presets && applicationWindow().showPassiveNotification(App.tr("“%1” is now used on “%2”.", preset.name, presets.host.item.title ?? "")));
    }

    spacing: 0
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "profiles")
                presets.load();
        }
    }

    Repeater {
        model: presets.all

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true
            spacing: Look.e2

            Fila {
                id: line

                Layout.fillWidth: true
                text: row.modelData.name
                note: presets.caption(row.modelData)
                explain: App.tr("Enter uses it on “%1”. It works for every scene.", presets.host.item.title ?? "")
                Keys.onReturnPressed: presets.use(row.modelData)
                Keys.onEnterPressed: presets.use(row.modelData)
            }

            QQC2.Button {
                id: drop

                flat: true
                icon.name: "x"
                text: App.tr("Delete…")
                Accessible.name: App.tr("Delete “%1”…", row.modelData.name)
                onClicked: {
                    presets.doomed = row.modelData;
                    confirm.opener = drop;
                    confirm.open();
                }
            }
        }
    }

    Fila {
        id: fresh

        Layout.fillWidth: true
        kind: "pliega"
        text: App.tr("New preset…")
        onOpened: maker.start(fresh)
    }

    PanelChico {
        id: confirm

        title: App.tr("Delete “%1”?", presets.doomed ? presets.doomed.name : "")
        text: App.tr("It leaves “My presets” and every scene that uses it. It cannot be undone.")
        quiet: App.tr("Cancel")
        principal: App.tr("Delete")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("DeletePreset", [presets.doomed.uid], "Profiles1")
    }

    PresetMaker {
        id: maker
    }
}
