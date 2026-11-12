import QtQuick
import QtQuick.Shapes
import org.kde.kirigami as Kirigami

Shape {
    id: area

    required property var loops
    required property real ratio
    property color ink: Kirigami.Theme.highlightColor
    property real fill: 0.2
    property bool dashed
    property bool stroked: true

    ShapePath {
        strokeColor: area.stroked ? area.ink : "transparent"
        strokeWidth: 2
        strokeStyle: area.dashed ? ShapePath.DashLine : ShapePath.SolidLine
        fillColor: area.fill > 0 ? Qt.rgba(area.ink.r, area.ink.g, area.ink.b, area.fill) : "transparent"
        fillRule: ShapePath.OddEvenFill

        PathMultiline {
            paths: (area.loops ?? []).filter(l => l.length > 1).map(l => l.concat([l[0]]).map(c => Qt.point(c[0] * area.ratio, c[1] * area.ratio)))
        }
    }
}
