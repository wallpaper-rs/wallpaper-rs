import QtQuick
import QtQuick.Dialogs

Item {
    id: choose

    required property var daemon

    FileDialog {
        id: files

        property string output: ""

        nameFilters: [i18n("Images and videos") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.mp4 *.m4v *.webm *.mkv *.mov *.avi *.ogv *.mpg *.mpeg)"]
        onAccepted: choose.daemon.act("SetWallpaper", [choose.daemon.path(selectedFile), output, ""])
    }

    Connections {
        target: choose.daemon

        function onChoose(name) {
            files.output = name;
            files.open();
        }
    }
}
