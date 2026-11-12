import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: smooth

    required property var host
    readonly property var value: host.field.value
    readonly property bool apart: typeof value === "object" && value !== null
    readonly property real up: Number(apart ? value.up : value) || 0
    readonly property real down: Number(apart ? value.down : value) || 0
    readonly property var response: JSON.parse(host.page.editor.rig("", JSON.stringify({
                "response": {
                    "smooth": smooth.apart ? {
                        "up": smooth.up,
                        "down": smooth.down
                    } : smooth.up
                }
            })) || "null") ?? ({})

    function round(seconds) {
        return Math.round(Math.min(60, Math.max(0, seconds)) * 100) / 100;
    }

    function put(rise, fall, gesture) {
        const next = apart ? {
                "up": round(rise),
                "down": round(fall)
            } : round(rise);
        host.commit(next, gesture);
    }

    function spot(point) {
        return Qt.point(point[0] / Math.max(1e-6, response.duration) * graph.width, (1 - point[1]) * (graph.height - 4) + 2);
    }

    onResponseChanged: graph.requestPaint()

    Canvas {
        id: graph

        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 4
        Accessible.name: App.tr("Response of the smoothing to a step up and down")
        onWidthChanged: requestPaint()
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            ctx.strokeStyle = Kirigami.Theme.disabledTextColor;
            ctx.strokeRect(0, 0, width, height);
            const points = smooth.response.points ?? [];
            if (points.length < 2)
                return;
            ctx.setLineDash([3, 3]);
            [smooth.response.rises, smooth.response.falls].forEach(at => {
                const x = smooth.spot([at, 0]).x;
                ctx.beginPath();
                ctx.moveTo(x, 0);
                ctx.lineTo(x, height);
                ctx.stroke();
            });
            ctx.setLineDash([]);
            ctx.strokeStyle = Kirigami.Theme.highlightColor;
            ctx.lineWidth = 2;
            ctx.beginPath();
            points.forEach((p, i) => {
                const s = smooth.spot(p);
                i ? ctx.lineTo(s.x, s.y) : ctx.moveTo(s.x, s.y);
            });
            ctx.stroke();
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: smooth.response.settles ? App.tr("Settles in %1 s going up and %2 s going down", smooth.response.settles[0].toFixed(2), smooth.response.settles[1].toFixed(2)) : ""
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
    }

    QQC2.Switch {
        text: App.tr("Different speed going up and down")
        checked: smooth.apart
        onToggled: smooth.host.commit(checked ? {
                "up": smooth.up,
                "down": smooth.down
            } : smooth.up)
    }

    Repeater {
        model: smooth.apart ? [[App.tr("Going up (s)"), true], [App.tr("Going down (s)"), false]] : [[App.tr("Seconds"), true]]

        delegate: RowLayout {
            id: row

            required property var modelData
            readonly property real now: modelData[1] ? smooth.up : smooth.down

            Layout.fillWidth: true

            QQC2.Label {
                text: row.modelData[0]
            }

            SceneSlider {
                Layout.fillWidth: true
                from: 0
                to: 5
                value: Math.min(5, row.now)
                onMoved: smooth.put(row.modelData[1] ? Math.round(value * 100) / 100 : smooth.up, row.modelData[1] ? smooth.down : Math.round(value * 100) / 100, true)
            }

            QQC2.TextField {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 4
                text: String(Number(row.now.toFixed(2)))
                onEditingFinished: {
                    const typed = smooth.host.parse(text, row.now);
                    smooth.put(row.modelData[1] ? typed : smooth.up, row.modelData[1] ? smooth.down : typed, false);
                }
            }
        }
    }
}
