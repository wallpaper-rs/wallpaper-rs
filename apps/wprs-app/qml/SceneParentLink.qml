import QtQuick
import QtQuick.Shapes
import org.kde.kirigami as Kirigami

Shape {
    id: link

    required property var page
    required property real ratio
    readonly property var gizmos: page.view.gizmos ?? []
    readonly property var chosen: gizmos.find(g => g.pointer === page.editor.selected) ?? null
    readonly property var leader: chosen?.parent ? gizmos.find(g => g.id === chosen.parent) ?? null : null

    visible: leader !== null

    ShapePath {
        strokeColor: Kirigami.Theme.highlightColor
        strokeWidth: 1
        strokeStyle: ShapePath.DashLine
        fillColor: "transparent"

        PathPolyline {
            path: link.leader ? [Qt.point(link.chosen.pivot[0] * link.ratio, link.chosen.pivot[1] * link.ratio), Qt.point(link.leader.pivot[0] * link.ratio, link.leader.pivot[1] * link.ratio)] : []
        }
    }
}
