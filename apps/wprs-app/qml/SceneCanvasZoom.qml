import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: zoom

    required property var canvas

    QQC2.ToolButton {
        icon.name: "zoom-out"
        text: App.tr("Zoom out")
        display: QQC2.AbstractButton.IconOnly
        onClicked: zoom.canvas.zoom = Math.max(0.1, zoom.canvas.zoom / 1.25)
    }

    QQC2.Label {
        text: Math.round(zoom.canvas.zoom * 100) + " %"
    }

    QQC2.ToolButton {
        icon.name: "zoom-in"
        text: App.tr("Zoom in")
        display: QQC2.AbstractButton.IconOnly
        onClicked: zoom.canvas.zoom = Math.min(16, zoom.canvas.zoom * 1.25)
    }

    QQC2.ToolButton {
        icon.name: "zoom-fit-best"
        text: App.tr("Fit")
        display: QQC2.AbstractButton.IconOnly
        onClicked: {
            zoom.canvas.zoom = 1;
            zoom.canvas.pan = Qt.point(0, 0);
        }
    }

    QQC2.ToolButton {
        checkable: true
        checked: zoom.canvas.testing
        icon.name: "input-mouse"
        text: App.tr("Test the pointer and clicks")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onToggled: {
            zoom.canvas.testing = checked;
            if (!checked)
                zoom.canvas.page.editor.touch(-1, -1, 0);
        }
    }
}
