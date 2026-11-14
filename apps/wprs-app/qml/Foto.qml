import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Image {
    id: foto

    property real corner: Look.radio
    property real lowCorner: corner

    asynchronous: true
    cache: true
    fillMode: Image.PreserveAspectCrop
    layer.enabled: status === Image.Ready && (corner > 0 || lowCorner > 0) && GraphicsInfo.api !== GraphicsInfo.Software && GraphicsInfo.api !== GraphicsInfo.Unknown
    layer.effect: Kirigami.ShadowedTexture {
        color: "transparent"
        radius: foto.corner
        corners.bottomLeftRadius: foto.lowCorner
        corners.bottomRightRadius: foto.lowCorner
    }
}
