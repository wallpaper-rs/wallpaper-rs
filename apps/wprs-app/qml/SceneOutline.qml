import QtQuick
import QtQuick.Shapes
import org.kde.kirigami as Kirigami

Shape {
    id: outline

    required property var corners
    required property real ratio
    property bool chosen
    property bool dashed

    ShapePath {
        strokeColor: outline.chosen ? Kirigami.Theme.highlightColor : Kirigami.Theme.disabledTextColor
        strokeWidth: outline.chosen ? 2 : 1
        strokeStyle: outline.dashed ? ShapePath.DashLine : ShapePath.SolidLine
        fillColor: "transparent"

        PathPolyline {
            path: (outline.corners ?? []).concat(outline.corners?.length ? [outline.corners[0]] : []).map(c => Qt.point(c[0] * outline.ratio, c[1] * outline.ratio))
        }
    }
}
