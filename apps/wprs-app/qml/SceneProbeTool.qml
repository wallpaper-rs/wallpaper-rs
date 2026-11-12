import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: tool

    required property var page

    icon.name: "input-mouse"
    text: App.tr("Try the parallax")
    display: QQC2.AbstractButton.IconOnly
    visible: (page.view.canvas?.parallax ?? null) !== null
    checkable: true
    checked: (page.view.probe ?? null) !== null
    onToggled: page.run("canvas", {
        "probe": checked ? {
            "pointer": [0.5, 0.5]
        } : {
            "off": true
        }
    })
    QQC2.ToolTip.text: App.tr("Move a simulated pointer, or the tilt of a phone, on the canvas and watch every layer shift by its parallax depth")
    QQC2.ToolTip.visible: hovered
}
