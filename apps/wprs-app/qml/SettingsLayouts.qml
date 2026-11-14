import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    property var chosen: null
    property Item opener: null
    property var presets: []

    function rules(profile) {
        const auto = profile.auto ?? {};
        const said = [];
        if ((auto.outputs ?? []).length > 0)
            said.push(App.tr("With %1", auto.outputs.join(" + ")));
        if (auto.on_battery === true)
            said.push(App.tr("on battery"));
        else if (auto.on_battery === false)
            said.push(App.tr("plugged in"));
        return said.length > 0 ? said.join(", ") : App.tr("Only when you choose it");
    }

    function choose(profile, row) {
        chosen = profile;
        opener = row;
        menu.opener = row;
        menu.open();
    }

    implicitHeight: filas.height
    Component.onCompleted: App.query("ListPresets", [""], "Profiles1", reply => part && (part.presets = reply.presets))

    Filas {
        id: filas

        Repeater {
            model: part.page.profiles

            Fila {
                id: layout

                required property var modelData

                objectName: "dynamic"
                width: parent.width
                text: modelData.name
                note: part.rules(modelData)
                value: modelData.uid === part.page.active ? App.tr("In use") : ""
                kind: "pliega"
                onOpened: part.choose(modelData, layout)
            }
        }

        Fila {
            id: save

            text: App.tr("Save this layout…")
            note: App.tr("What each screen shows now, under a name")
            kind: "pliega"
            onOpened: saver.ask(save)
        }

        Fila {
            text: App.tr("No layout")
            note: App.tr("Stop following a layout; what is on now stays")
            kind: "pliega"
            value: part.page.active === "" ? App.tr("In use") : ""
            onOpened: App.act("Activate", [""], "Profiles1")
        }

        Seccion {
            text: App.tr("Presets for every wallpaper")
        }

        Repeater {
            model: part.presets

            Fila {
                required property var modelData

                objectName: "dynamic"
                width: parent.width
                text: modelData.name
                note: [Labels.value(modelData.value.fit), modelData.value.background].filter(word => !!word).join(" · ")
                kind: "pliega"
                value: App.tr("Delete")
                onOpened: App.act("DeletePreset", [modelData.uid], "Profiles1")
            }
        }

        Fila {
            id: making

            text: App.tr("New preset…")
            kind: "pliega"
            onOpened: {
                preset.opener = making;
                preset.open();
            }
        }
    }

    ListaMenu {
        id: menu

        options: [
            {
                "text": App.tr("Use it"),
                "icon": "play"
            },
            {
                "text": App.tr("Automatic rules"),
                "icon": "magic-wand"
            },
            {
                "text": App.tr("Change the name…"),
                "icon": "pencil-simple"
            },
            {
                "text": App.tr("Delete it…"),
                "icon": "trash",
                "tone": "coral"
            }
        ]
        onChosen: index => {
            const profile = part.chosen;
            if (!profile)
                return;
            if (index === 0)
                App.act("Activate", [profile.uid], "Profiles1");
            else
                Qt.callLater(() => [rules, renamer, confirm][index - 1].open());
        }
    }

    SettingsLayoutRules {
        id: rules

        opener: part.opener
        profile: part.chosen
    }

    SettingsLayoutName {
        id: renamer

        opener: part.opener
        profile: part.chosen
    }

    SettingsLayoutSave {
        id: saver

        page: part.page
    }

    PanelChico {
        id: confirm

        opener: part.opener
        title: part.chosen ? App.tr("Delete the layout “%1”?", part.chosen.name) : ""
        text: App.tr("The wallpapers and the screens stay as they are now.")
        quiet: App.tr("Cancel")
        principal: App.tr("Delete")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("Delete", [part.chosen.uid], "Profiles1")
    }

    PresetDialog {
        id: preset
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "profiles")
                App.query("ListPresets", [""], "Profiles1", reply => part && (part.presets = reply.presets));
        }
    }
}
