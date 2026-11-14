import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: dialogs

    required property var host
    visible: false
    readonly property alias keys: keys
    readonly property alias filer: filer
    readonly property alias wallhaven: wallhaven

    SourceKeyPanel {
        id: keys

        opener: dialogs.host
    }

    SecretFiler {
        id: filer
    }

    Loader {
        id: wallhaven

        function open() {
            active = true;
        }

        active: false
        sourceComponent: WallhavenDialog {
            provider: host.chosen
            kind: (App.schema.providers || []).find(p => p.kind === host.chosen.kind) || ({})
            Component.onCompleted: open()
            onClosed: wallhaven.active = false
        }
    }
}
