import QtQuick
import QtQuick.Controls as QQC2

QQC2.Button {
    required property var host

    visible: host.info.blank !== true
    text: host.daemon.tr("Remove wallpaper")
    icon.name: "edit-clear"
    QQC2.ToolTip.text: host.daemon.state?.plasma === true ? host.daemon.tr("Show only the background color. You can bring the wallpaper back.") : host.daemon.tr("Give the screen back to the desktop's own wallpaper. You can bring the wallpaper back.")
    QQC2.ToolTip.visible: hovered
    onClicked: host.daemon.act("ClearWallpaper", [host.name])
}
