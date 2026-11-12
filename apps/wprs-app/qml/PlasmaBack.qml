import QtQuick
import QtQuick.Controls as QQC2

QQC2.Button {
    required property var host

    visible: host.daemon.state?.plasma === true && host.name === ""
    text: host.daemon.tr("Use Plasma's own wallpaper")
    icon.name: "preferences-desktop-wallpaper"
    QQC2.ToolTip.text: host.daemon.tr("Switch the screens back to the wallpaper type you had in Plasma. Choosing a wallpaper here uses wallpaper-rs again.")
    QQC2.ToolTip.visible: hovered
    onClicked: host.daemon.act("HandBackToPlasma", [])
}
