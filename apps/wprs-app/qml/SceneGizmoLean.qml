import QtQuick

SceneOutline {
    required property var gizmo

    parent: gizmo.parent
    anchors.fill: parent
    z: gizmo.z
    visible: gizmo.modelData.corners !== null
    corners: gizmo.modelData.corners ?? []
    ratio: gizmo.ratio
    chosen: gizmo.chosen
    dashed: true
}
