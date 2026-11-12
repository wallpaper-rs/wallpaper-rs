import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: capture

    required property var host
    readonly property bool animated: host.field.widget === "ref:animated"
    readonly property real second: Math.max(0, Number(host.page.editor.time ?? 0))

    visible: (host.page.view.kind ?? "scene") === "scene"
    enabled: !host.page.editor.busy
    icon.name: animated ? "media-record" : "camera-photo"
    text: animated ? App.tr("Record a preview from the scene") : App.tr("Capture the thumbnail from the scene")
    display: QQC2.AbstractButton.IconOnly
    QQC2.ToolTip.text: animated ? App.tr("Record 4 seconds of the scene from the second the preview is on") : App.tr("Draw the scene at the second the preview is on and keep that frame")
    QQC2.ToolTip.visible: hovered
    onClicked: host.page.run("shot", animated ? {
            "recording": {
                "from": second,
                "seconds": 4,
                "fps": 10
            }
        } : {
            "still": {
                "time": second
            }
        })
}
