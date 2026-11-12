import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
    id: capture

    required property var host
    readonly property bool animated: host.field.widget === "ref:animated"
    readonly property real second: Math.max(0, Number(host.page.editor.time ?? 0))

    property bool webp: false
    readonly property string kind: host.page.view.kind ?? "scene"

    visible: kind === "scene" || kind === "video" || (kind === "image" && !animated)
    enabled: !host.page.editor.busy
    icon.name: animated ? "media-record" : "camera-photo"
    text: animated ? (webp ? App.tr("Record a preview as WebP") : App.tr("Record a preview as GIF")) : App.tr("Capture the thumbnail")
    display: QQC2.AbstractButton.IconOnly
    QQC2.ToolTip.text: animated ? App.tr("Record 4 seconds from the second the preview is on, as a GIF or, with the WebP box ticked, as a WebP that keeps every color") : kind === "image" ? App.tr("Keep the picture of the package, made smaller, as the thumbnail") : App.tr("Draw the second the preview is on and keep that frame")
    QQC2.ToolTip.visible: hovered
    onClicked: host.page.run("shot", animated ? {
            "recording": {
                "from": second,
                "seconds": 4,
                "fps": 10,
                "format": webp ? "webp" : "gif"
            }
        } : {
            "still": {
                "time": second
            }
        })
}
