import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: tool

    required property var page
    readonly property var camera: page.view.camera2d ?? null

    function camera2d(edit) {
        page.run("canvas", edit);
    }

    icon.name: "camera-video"
    text: App.tr("2D camera")
    display: QQC2.AbstractButton.IconOnly
    checkable: true
    checked: camera?.whole ?? false
    onClicked: {
        checked = Qt.binding(() => tool.camera?.whole ?? false);
        menu.open();
    }
    QQC2.ToolTip.text: camera ? (camera.whole ? App.tr("The canvas shows the whole scene with the frame of the camera") : App.tr("The canvas shows the scene through the camera")) : App.tr("Add a 2D camera that zooms, turns and moves the view")
    QQC2.ToolTip.visible: hovered

    QQC2.Menu {
        id: menu

        QQC2.MenuItem {
            text: App.tr("Add a 2D camera")
            visible: tool.camera === null
            height: visible ? implicitHeight : 0
            onTriggered: tool.camera2d({
                "camera": {
                    "zoom": 1
                }
            })
        }

        QQC2.MenuItem {
            text: App.tr("View through the camera")
            visible: tool.camera !== null
            height: visible ? implicitHeight : 0
            checkable: true
            checked: !(tool.camera?.whole ?? true)
            onTriggered: tool.camera2d({
                "camera_view": {
                    "whole": false
                }
            })
        }

        QQC2.MenuItem {
            text: App.tr("View the whole canvas with the camera frame")
            visible: tool.camera !== null
            height: visible ? implicitHeight : 0
            checkable: true
            checked: tool.camera?.whole ?? false
            onTriggered: tool.camera2d({
                "camera_view": {
                    "whole": true
                }
            })
        }

        QQC2.MenuItem {
            text: App.tr("Remove the camera")
            visible: tool.camera !== null
            height: visible ? implicitHeight : 0
            onTriggered: tool.camera2d({
                "camera": {
                    "off": true
                }
            })
        }
    }
}
