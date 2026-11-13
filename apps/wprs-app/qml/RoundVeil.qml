import QtQuick
import QtQuick.Shapes
import io.github.wallpaper_rs.app

Item {
    id: veil

    property real centerX: 0
    property real centerY: 0.5
    property real reachX: width
    property real reachY: height
    property real inner: 1
    property real middle: 1
    property real middleAt: 0.6
    readonly property real squash: reachX > 0 ? reachY / reachX : 1

    function shade(alpha) {
        return Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, Math.max(0, Math.min(1, alpha)));
    }

    Shape {
        width: veil.width
        height: veil.squash > 0 ? veil.height / veil.squash : 0
        transform: Scale {
            yScale: veil.squash
        }

        ShapePath {
            strokeWidth: -1
            strokeColor: "transparent"
            startX: 0
            startY: 0
            fillGradient: RadialGradient {
                centerX: veil.centerX * veil.width
                centerY: veil.centerY * veil.height / veil.squash
                centerRadius: veil.reachX
                focalX: centerX
                focalY: centerY
                focalRadius: 0

                GradientStop {
                    position: 0
                    color: veil.shade(veil.inner)
                }

                GradientStop {
                    position: veil.middleAt
                    color: veil.shade(veil.middle)
                }

                GradientStop {
                    position: 1
                    color: veil.shade(0)
                }
            }

            PathLine {
                x: veil.width
                y: 0
            }

            PathLine {
                x: veil.width
                y: veil.height / veil.squash
            }

            PathLine {
                x: 0
                y: veil.height / veil.squash
            }

            PathLine {
                x: 0
                y: 0
            }
        }
    }
}
