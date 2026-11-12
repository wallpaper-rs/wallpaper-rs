import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: thumb

    required property var page
    property string layerId
    property int index: -1
    property bool active: true
    readonly property var shown: active && layerId ? (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
                "mask_view": {
                    "layer": layerId,
                    "index": index >= 0 ? index : null,
                    "time": page.editor.time
                }
            })) || "null")) : null
    readonly property real span: Kirigami.Units.gridUnit * 7
    readonly property real tall: shown ? Math.max(Kirigami.Units.gridUnit * 2, Math.min(span, span * shown.size[1] / Math.max(1, shown.size[0]))) : 0
    readonly property var picture: shown?.texture ? JSON.parse(page.editor.rig("", JSON.stringify({
                "mask_picture": {
                    "asset": shown.texture,
                    "channel": shown.channel ?? null,
                    "invert": shown.invert,
                    "strength": shown.strength ?? 1,
                    "tint": "#ffffff"
                }
            })) || "null") : null
    readonly property var loops: {
        if (!shown || shown.area_u.length === 0)
            return [];
        const scaled = shown.area_u.map(l => l.map(p => [p[0] * span, p[1] * tall]));
        return shown.invert ? [[[0, 0], [span, 0], [span, tall], [0, tall]]].concat(scaled) : scaled;
    }

    implicitWidth: span
    implicitHeight: tall

    Rectangle {
        anchors.fill: parent
        color: Qt.rgba(0, 0, 0, 0.85)
        border.color: Kirigami.Theme.disabledTextColor
    }

    Image {
        anchors.fill: parent
        visible: thumb.picture !== null
        fillMode: Image.Stretch
        asynchronous: true
        source: thumb.picture ? "file://" + thumb.picture.path : ""
    }

    SceneArea {
        loops: thumb.loops
        ratio: 1
        fill: 0.6
        stroked: false
    }

    QQC2.Label {
        anchors.centerIn: parent
        visible: thumb.shown?.source === "layer"
        text: thumb.shown?.layer ?? ""
        color: "white"
    }
}
