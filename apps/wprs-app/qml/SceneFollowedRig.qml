import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Item {
    id: followed

    required property var page
    required property real ratio
    readonly property var chosen: (page.view.gizmos ?? []).find(g => g.pointer === page.editor.selected) ?? null
    readonly property var rows: page.mesh.following !== "" && chosen ? (page.view.followed ?? []).filter(g => g.id === page.mesh.following) : []

    anchors.fill: parent

    SceneRepeater {
        id: listed

        rows: followed.rows

        delegate: Item {
            id: box

            required property int index
            readonly property var modelData: listed.rows[index]
            readonly property var attached: followed.page.view && followed.chosen ? JSON.parse(followed.page.editor.value(followed.chosen.pointer + "/attach") || "null") : null
            property var info: ({
                    "bones": [],
                    "points": []
                })
            readonly property var spots: info.bones.map(b => ({
                        "kind": "bone",
                        "name": b.name,
                        "at": b.at
                    })).concat(info.points.filter(p => !p.part).map(p => ({
                        "kind": "point",
                        "name": p.name,
                        "at": p.at
                    }))).filter(s => s.at)

            function load() {
                info = JSON.parse(followed.page.editor.rig(modelData.id, "{}") || "null") ?? ({
                        "bones": [],
                        "points": []
                    });
            }

            x: (modelData.offset[0] + modelData.position[0]) * followed.ratio - width / 2
            y: (modelData.offset[1] + modelData.position[1]) * followed.ratio - height / 2
            width: Math.max(8, modelData.extent[0] * followed.ratio)
            height: Math.max(8, modelData.extent[1] * followed.ratio)
            rotation: modelData.angle
            Component.onCompleted: load()

            Connections {
                function onViewChanged() {
                    box.load();
                }

                target: followed.page.editor
            }

            SceneRepeater {
                id: dots

                rows: box.spots

                delegate: Rectangle {
                    id: knob

                    required property int index
                    readonly property var modelData: dots.rows[index]
                    readonly property bool followedNow: box.attached !== null && box.attached.layer === box.modelData.id && (box.attached[modelData.kind] ?? "") === modelData.name

                    x: modelData.at[0] * box.width - width / 2
                    y: modelData.at[1] * box.height - height / 2
                    width: Kirigami.Units.gridUnit * (followedNow ? 0.9 : 0.65)
                    height: width
                    radius: modelData.kind === "bone" ? width / 2 : 0
                    color: followedNow ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.highlightColor
                    border.color: Kirigami.Theme.backgroundColor
                    opacity: hover.hovered || followedNow ? 1 : 0.7
                    QQC2.ToolTip.visible: hover.hovered
                    QQC2.ToolTip.text: modelData.name

                    HoverHandler {
                        id: hover

                        cursorShape: Qt.PointingHandCursor
                    }

                    TapHandler {
                        gesturePolicy: TapHandler.ReleaseWithinBounds
                        onTapped: {
                            const args = {
                                "layer": followed.chosen.id,
                                "to": box.modelData.id
                            };
                            args[knob.modelData.kind] = knob.modelData.name;
                            followed.page.run("attach", args);
                        }
                    }
                }
            }
        }
    }
}
