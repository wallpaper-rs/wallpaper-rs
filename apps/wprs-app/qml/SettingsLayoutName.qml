import QtQuick
import io.github.wallpaper_rs.app

PanelChico {
    id: panel

    property var profile: null

    title: App.tr("New name for the layout")
    quiet: App.tr("Cancel")
    principal: App.tr("Rename")
    principalIcon: "pencil-simple"
    onAccepted: panel.profile && name.fieldText !== "" && App.act("Rename", [panel.profile.uid, name.fieldText], "Profiles1")
    onProfileChanged: name.fieldText = profile ? profile.name : ""

    Fila {
        id: name

        width: parent ? parent.width : implicitWidth
        text: App.tr("Name")
        kind: "campo"
        placeholder: App.tr("No name")
    }
}
