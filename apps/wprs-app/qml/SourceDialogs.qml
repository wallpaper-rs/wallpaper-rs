import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var host
    visible: false
    readonly property alias keys: keys
    readonly property alias filer: filer

    SourceKeyPanel {
        id: keys

        opener: dialogs.host
    }

    SecretFiler {
        id: filer
    }
}
