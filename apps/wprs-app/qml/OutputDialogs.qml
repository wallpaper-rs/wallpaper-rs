import QtQuick
import QtQuick.Dialogs

Item {
    id: dialogs

    required property var host
    readonly property alias files: files
    readonly property alias colors: colors

    FileDialog {
        id: files

        nameFilters: [dialogs.host.daemon.tr("Images and videos") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.mp4 *.m4v *.webm *.mkv *.mov *.avi *.ogv *.mpg *.mpeg)"]
        onAccepted: dialogs.host.daemon.act("SetWallpaper", [dialogs.host.daemon.path(selectedFile), dialogs.host.name, ""])
    }

    ColorDialog {
        id: colors

        onAccepted: dialogs.host.daemon.act("SetBackground", [selectedColor.toString(), dialogs.host.name])
    }
}
