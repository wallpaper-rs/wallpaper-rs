import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tool

    required property var page
    required property string map
    readonly property var relief: page.relief
    readonly property var chart: JSON.parse(relief.regions || "{}")
    property var zones: chart.zones ?? []
    readonly property var survey: JSON.parse(relief.survey || "{}")
    readonly property string key: JSON.stringify(page.edit.depth ?? {})
    readonly property var picks: page.regionPicks
    readonly property int edits: (survey.joins ?? 0) + (survey.splits ?? 0) + (survey.shifts ?? 0)
    property real step: 0.1

    function zone(order) {
        return zones.find(z => z.order === order);
    }

    function spot(order) {
        const z = zone(order);
        return z ? z.at[0].toFixed(4) + "," + z.at[1].toFixed(4) : "";
    }

    function refresh() {
        if (map === "")
            return;
        relief.regions_of(page.editor.json);
        relief.measure(page.editor.json);
    }

    function join() {
        const points = picks.map(spot).filter(p => p !== "");
        page.regionPicks = [];
        page.regionLast = "depth_join";
        if (points.length >= 2)
            page.run("depth_join", {
                "points": points.join(";")
            });
    }

    function nudge(by) {
        const spots = picks.map(spot).filter(p => p !== "");
        page.regionPicks = [];
        page.regionLast = "depth_move";
        spots.forEach(at => page.run("depth_move", {
                "at": at,
                "by": by
            }));
    }

    function reset() {
        page.regionPicks = [];
        page.regionLast = "depth_move";
        page.run("depth_join", {
            "clear": true
        });
        page.run("depth_split", {
            "clear": true
        });
        page.run("depth_move", {
            "clear": true
        });
    }

    onKeyChanged: {
        refresh();
        if (visible && map !== "")
            relief.sync(page.editor.json);
    }
    onMapChanged: refresh()
    onVisibleChanged: refresh()
    Component.onCompleted: refresh()

    Connections {
        target: tool.relief

        function onBuiltChanged() {
            tool.refresh();
        }
    }

    Kirigami.Separator {
        Layout.fillWidth: true
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Regions")
    }

    QQC2.Switch {
        text: App.tr("Show the regions on the picture")
        checked: tool.page.regionsOn
        onToggled: tool.page.regionsOn = checked
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("%1 regions, the nearest first. %2 joined, %3 split, %4 moved.", tool.zones.length, tool.survey.joins ?? 0, tool.survey.splits ?? 0, tool.survey.shifts ?? 0)
        wrapMode: Text.Wrap
    }

    EditRegionRows {
        Layout.fillWidth: true
        page: tool.page
        zones: tool.zones
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.Button {
            icon.name: "edit-link"
            text: App.tr("Join")
            enabled: tool.picks.length >= 2
            onClicked: tool.join()
        }

        QQC2.Button {
            icon.name: "arrow-up"
            text: App.tr("Nearer")
            enabled: tool.picks.length >= 1
            onClicked: tool.nudge(tool.step)
        }

        QQC2.Button {
            icon.name: "arrow-down"
            text: App.tr("Farther")
            enabled: tool.picks.length >= 1
            onClicked: tool.nudge(-tool.step)
        }
    }

    EditSlider {
        text: App.tr("Move by")
        from: 0.01
        to: 0.5
        stepSize: 0.01
        value: tool.step
        onMoved: value => tool.step = value
    }

    QQC2.Button {
        Layout.fillWidth: true
        checkable: true
        checked: tool.page.regionSplit
        icon.name: "edit-cut"
        text: App.tr("Split along a stroke")
        onClicked: tool.page.regionSplit = checked
    }

    RowLayout {
        QQC2.ToolButton {
            icon.name: "edit-undo"
            text: App.tr("Undo")
            enabled: tool.page.editor.canUndo
            onClicked: tool.page.editor.undo()
        }

        QQC2.ToolButton {
            icon.name: "edit-redo"
            text: App.tr("Redo")
            enabled: tool.page.editor.canRedo
            onClicked: tool.page.editor.redo()
        }

        QQC2.ToolButton {
            icon.name: "edit-clear"
            text: App.tr("Put the regions back")
            enabled: tool.edits > 0
            onClicked: tool.reset()
        }
    }

    EditRegionNote {
        Layout.fillWidth: true
        survey: tool.survey
        last: tool.page.regionLast
    }

    EditRegionPreview {
        Layout.fillWidth: true
        page: tool.page
        ready: tool.survey.built ?? false
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("A region is a part of the picture between depth jumps; each moves as one piece. Click regions on the picture or tick them here, then join them or move them nearer or farther. To cut a region in two, turn on Split along a stroke and draw across it.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }
}
