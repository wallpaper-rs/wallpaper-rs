import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Loader {
    id: sombra

    property real corner: Look.radio
    property string kind: "tapa"
    readonly property bool hondo: GraphicsInfo.api !== GraphicsInfo.Software && GraphicsInfo.api !== GraphicsInfo.Unknown
    readonly property var layers: ({
            "tapa": [[18, 40, 0.55, Look.noche], [4, 10, 0.4, Look.noche]],
            "foco": [[26, 60, 0.7, Look.noche], [6, 14, 0.5, Look.noche]],
            "panel": [[24, 64, 0.45, "black"], [4, 12, 0.3, "black"]],
            "fila": [[10, 28, 0.35, "black"]]
        })[kind] ?? []

    anchors.fill: parent
    z: -1
    active: hondo

    sourceComponent: Item {
        Repeater {
            model: sombra.layers

            Kirigami.ShadowedRectangle {
                required property var modelData
                readonly property color tone: modelData[3]

                anchors.fill: parent
                radius: sombra.corner
                color: "transparent"
                shadow.xOffset: 0
                shadow.yOffset: modelData[0]
                shadow.size: modelData[1]
                shadow.color: Qt.rgba(tone.r, tone.g, tone.b, modelData[2])
            }
        }
    }
}
