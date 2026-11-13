import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page
    required property var zones
    readonly property var picks: page.regionPicks
    property real cut: 0.5
    property bool straight

    function aimLine() {
        const z = straight && picks.length === 1 ? zones.find(r => r.order === picks[0]) : undefined;
        if (!z) {
            page.regionLine = null;
            return;
        }
        const b = z.bounds, wide = b[2] - b[0] >= b[3] - b[1];
        const edge = v => Math.max(0, Math.min(1, v));
        const at = (wide ? b[1] : b[0]) + cut * (wide ? b[3] - b[1] : b[2] - b[0]);
        page.regionLine = wide ? [[edge(b[0] - 0.01), at], [edge(b[2] + 0.01), at]] : [[at, edge(b[1] - 0.01)], [at, edge(b[3] + 0.01)]];
    }

    function cutStraight() {
        const points = page.regionLine;
        page.regionLine = null;
        straight = false;
        page.regionPicks = [];
        if (points)
            page.run("depth_split", {
                "points": points.map(p => p[0].toFixed(4) + "," + p[1].toFixed(4)).join(";")
            });
    }

    onStraightChanged: aimLine()
    onCutChanged: aimLine()
    onPicksChanged: aimLine()
    onZonesChanged: aimLine()

    QQC2.Switch {
        text: App.tr("Straight split of the picked region")
        enabled: panel.picks.length === 1
        checked: panel.straight && panel.picks.length === 1
        onToggled: panel.straight = checked
    }

    EditSlider {
        visible: panel.straight && panel.picks.length === 1
        text: App.tr("Where to cut")
        from: 0.1
        to: 0.9
        stepSize: 0.05
        value: panel.cut
        onMoved: value => panel.cut = value
    }

    QQC2.Button {
        Layout.fillWidth: true
        visible: panel.straight && panel.picks.length === 1
        icon.name: "edit-cut"
        text: App.tr("Cut along the line")
        onClicked: panel.cutStraight()
    }
}
