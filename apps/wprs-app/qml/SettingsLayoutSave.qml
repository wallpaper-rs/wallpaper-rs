import QtQuick
import io.github.wallpaper_rs.app

PanelChico {
    id: panel

    required property var page

    function ask(from) {
        opener = from;
        name.fieldText = "";
        open();
    }

    function save(text, settings, outputs) {
        App.act("Save", [text, settings], "Profiles1", reply => panel && outputs.length > 0 && App.act("SetAuto", [reply[0], "outputs", outputs.join(",")], "Profiles1"));
    }

    title: App.tr("Save this layout")
    text: App.tr("It keeps what each screen shows now. It comes back by itself when these same screens are connected.")
    quiet: App.tr("Cancel")
    principal: App.tr("Save")
    principalIcon: "floppy-disk"
    onAccepted: save(name.fieldText !== "" ? name.fieldText : App.tr("Layout %1", panel.page.profiles.length + 1), keep.checked, panel.page.outputs)

    Filas {
        Fila {
            id: name

            text: App.tr("Name")
            kind: "campo"
            placeholder: App.tr("Layout %1", panel.page.profiles.length + 1)
        }

        Fila {
            id: keep

            text: App.tr("Include the settings")
            note: App.tr("Quality, pauses and transitions come back with it")
            kind: "llave"
            checked: false
            value: checked ? App.tr("Yes") : App.tr("No")
        }
    }
}
