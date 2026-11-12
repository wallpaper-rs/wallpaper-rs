import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: events

    required property var line
    readonly property var list: line.shown?.events ?? []

    function edit(event, gesture) {
        line.page.run("clip_event", Object.assign({
            "layer": line.mesh.layer,
            "clip": line.mesh.clip
        }, event), gesture);
    }

    function fresh() {
        const names = list.map(e => e.name);
        let n = list.length + 1;
        while (names.includes("event_" + n))
            n++;
        return "event_" + n;
    }

    visible: line.shown !== null
    height: Kirigami.Units.gridUnit * 1.6

    QQC2.Label {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 7
        text: App.tr("Events")
        font: Kirigami.Theme.smallFont
    }

    Item {
        id: track

        Layout.fillWidth: true
        Layout.fillHeight: true

        QQC2.Label {
            anchors.centerIn: parent
            visible: events.list.length === 0
            text: App.tr("Double-click the lane to add an event")
            font: Kirigami.Theme.smallFont
            opacity: 0.6
        }

        TapHandler {
            gesturePolicy: TapHandler.ReleaseWithinBounds
            onDoubleTapped: point => events.edit({
                    "name": events.fresh(),
                    "time": Math.round(Math.max(0, Math.min(1, point.position.x / track.width)) * events.line.length * 100) / 100
                })
        }

        SceneRepeater {
            id: listed

            rows: events.list

            delegate: SceneClipEventMark {
                required property int index

                lane: track
                owner: events
                event: listed.rows[index]
            }
        }
    }
}
