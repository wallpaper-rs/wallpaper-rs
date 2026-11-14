import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

PanelChico {
    id: panel

    property var profile: null

    title: profile ? App.tr("When “%1” comes by itself", profile.name) : ""
    text: App.tr("The layout is used on its own when these are true. With nothing set, only when you choose it.")
    quiet: App.tr("Close")

    Kirigami.FormLayout {
        width: parent ? parent.width : implicitWidth

        Settings {
            daemon: App
            model: App.schema.auto ?? []
            values: panel.profile ? Object.assign({
                "priority": panel.profile.priority
            }, panel.profile.auto) : ({})
            onEdited: (key, value) => panel.profile && App.act("SetAuto", [panel.profile.uid, key, value], "Profiles1")
        }
    }
}
