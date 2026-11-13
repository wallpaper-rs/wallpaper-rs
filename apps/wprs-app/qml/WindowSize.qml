import QtQuick
import QtCore as Core
import io.github.wallpaper_rs.app

QtObject {
    id: size

    required property var host
    property bool ready: false
    readonly property real areaWidth: host.screen?.desktopAvailableWidth || 1600
    readonly property real areaHeight: host.screen?.desktopAvailableHeight || 1000
    readonly property Core.Settings saved: Core.Settings {
        property int width: 0
        property int height: 0

        location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/wallpaper-rs/window.conf"
        category: "window"
    }
    readonly property Connections watch: Connections {
        target: size.host

        function onWidthChanged() {
            size.keep();
        }

        function onHeightChanged() {
            size.keep();
        }
    }

    function restore() {
        if (App.checking) {
            host.width = 1600;
            host.height = 900;
            return;
        }
        host.width = saved.width > 0 ? Math.min(saved.width, areaWidth) : Math.min(1600, Math.round(areaWidth * 0.9));
        host.height = saved.height > 0 ? Math.min(saved.height, areaHeight) : Math.min(1000, Math.round(areaHeight * 0.9));
        ready = true;
    }

    function roomy() {
        if (areaWidth < 1280 || areaHeight < 800)
            return host.showMaximized();
        if (host.visibility !== Window.Windowed)
            return;
        host.width = Math.max(host.width, Math.min(1440, Math.round(areaWidth * 0.9)));
        host.height = Math.max(host.height, Math.min(900, Math.round(areaHeight * 0.9)));
    }

    function keep() {
        if (!ready || App.checking || host.visibility !== Window.Windowed)
            return;
        saved.width = host.width;
        saved.height = host.height;
    }
}
