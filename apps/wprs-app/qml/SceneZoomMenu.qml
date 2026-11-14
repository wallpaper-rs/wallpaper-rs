import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: zoom

    required property var canvas
    readonly property var steps: [0.25, 0.5, 1, 2, 4]

    function set(value) {
        canvas.zoom = Math.max(0.1, Math.min(16, value));
    }

    icon.name: "magnifying-glass"
    text: Math.round(canvas.zoom * 100) + " %"
    Accessible.name: App.tr("Zoom: %1", text)
    onClicked: menu.popup(zoom, 0, zoom.height)

    QQC2.Menu {
        id: menu


        QQC2.MenuItem {
            icon.name: "magnifying-glass-plus"
            text: App.tr("Zoom in")
            onTriggered: zoom.set(zoom.canvas.zoom * 1.25)
        }

        QQC2.MenuItem {
            icon.name: "magnifying-glass-minus"
            text: App.tr("Zoom out")
            onTriggered: zoom.set(zoom.canvas.zoom / 1.25)
        }

        QQC2.MenuItem {
            icon.name: "frame-corners"
            text: App.tr("Fit")
            onTriggered: {
                zoom.canvas.zoom = 1;
                zoom.canvas.pan = Qt.point(0, 0);
            }
        }

        QQC2.MenuSeparator {}

        Repeater {
            model: zoom.steps

            delegate: QQC2.MenuItem {
                required property real modelData

                text: Math.round(modelData * 100) + " %"
                checkable: true
                checked: Math.abs(zoom.canvas.zoom - modelData) < 0.001
                onTriggered: zoom.set(modelData)
            }
        }
    }
}
