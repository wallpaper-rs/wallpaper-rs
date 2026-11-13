import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: canvas

    required property var page
    readonly property var relief: page.relief
    readonly property var chart: JSON.parse(relief.regions || "{}")
    property var zones: chart.zones ?? []
    property var grid: chart.grid ?? ({
            "w": 0,
            "h": 0,
            "cells": []
        })
    readonly property bool previewing: page.regionPreview && relief.frame !== ""
    readonly property int most: 1000
    property var stroke: []

    function pickAt(x, y) {
        if (grid.w === 0 || width <= 0 || height <= 0)
            return 0;
        const cx = Math.max(0, Math.min(grid.w - 1, Math.floor(x / width * grid.w)));
        const cy = Math.max(0, Math.min(grid.h - 1, Math.floor(y / height * grid.h)));
        return (grid.cells[cy * grid.w + cx] ?? -1) + 1;
    }

    function toggle(order) {
        if (order < 1)
            return;
        const has = page.regionPicks.includes(order);
        const rest = page.regionPicks.filter(o => o !== order);
        page.regionPicks = has ? rest : rest.concat([order]).sort((a, b) => a - b);
    }

    function add(x, y) {
        const to = [x / width, y / height].map(n => Math.max(0, Math.min(1, n)));
        const last = stroke[stroke.length - 1];
        if (last && Math.hypot(to[0] - last[0], to[1] - last[1]) <= 0.003)
            return;
        stroke = stroke.concat([to]);
        if (stroke.length >= most) {
            lay();
            stroke = [to];
        }
        line.requestPaint();
    }

    function lay() {
        const points = stroke;
        stroke = [];
        line.requestPaint();
        if (points.length < 2)
            return;
        page.regionLast = "depth_split";
        page.run("depth_split", {
                "points": points.map(p => p[0].toFixed(4) + "," + p[1].toFixed(4)).join(";")
            });
    }

    function aim(x, y) {
        page.pointer = [Math.max(-1, Math.min(1, x / width * 2 - 1)), Math.max(-1, Math.min(1, y / height * 2 - 1))];
    }

    onGridChanged: marks.requestPaint()
    Connections {
        target: canvas.page

        function onRegionPicksChanged() {
            marks.requestPaint();
        }

        function onRegionLineChanged() {
            line.requestPaint();
        }
    }

    Image {
        anchors.fill: parent
        visible: canvas.page.regionsOn && !canvas.previewing
        opacity: 0.8
        source: canvas.relief.chart
        fillMode: Image.Stretch
        cache: false
    }

    Canvas {
        id: marks

        anchors.fill: parent
        visible: canvas.page.regionsOn && !canvas.previewing
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            const g = canvas.grid, picks = canvas.page.regionPicks;
            if (g.w === 0 || picks.length === 0)
                return;
            ctx.fillStyle = Qt.rgba(1, 1, 1, 0.55);
            const cw = width / g.w, ch = height / g.h;
            for (let i = 0; i < g.cells.length; i++) {
                if (picks.includes(g.cells[i] + 1))
                    ctx.fillRect((i % g.w) * cw, Math.floor(i / g.w) * ch, Math.ceil(cw), Math.ceil(ch));
            }
        }
    }

    Repeater {
        model: canvas.page.regionsOn && !canvas.previewing ? canvas.zones : []

        delegate: Rectangle {
            required property var modelData

            x: modelData.at[0] * canvas.width - width / 2
            y: modelData.at[1] * canvas.height - height / 2
            width: Kirigami.Units.gridUnit * 1.4
            height: width
            radius: width / 2
            color: canvas.page.regionPicks.includes(modelData.order) ? Kirigami.Theme.highlightColor : Kirigami.Theme.backgroundColor
            border.color: Kirigami.Theme.textColor

            Text {
                anchors.centerIn: parent
                text: parent.modelData.order
                color: canvas.page.regionPicks.includes(parent.modelData.order) ? Kirigami.Theme.highlightedTextColor : Kirigami.Theme.textColor
            }
        }
    }

    Image {
        anchors.fill: parent
        visible: canvas.previewing
        source: canvas.relief.frame
        fillMode: Image.Stretch
        cache: false
    }

    Canvas {
        id: line

        anchors.fill: parent
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            const straight = canvas.page.regionLine;
            ctx.strokeStyle = Kirigami.Theme.negativeTextColor;
            if (straight) {
                ctx.lineWidth = 3;
                ctx.setLineDash([8, 6]);
                ctx.beginPath();
                ctx.moveTo(straight[0][0] * width, straight[0][1] * height);
                ctx.lineTo(straight[1][0] * width, straight[1][1] * height);
                ctx.stroke();
                ctx.setLineDash([]);
            }
            if (canvas.stroke.length === 0)
                return;
            ctx.lineWidth = 3;
            ctx.lineCap = "round";
            ctx.beginPath();
            canvas.stroke.forEach((p, i) => i === 0 ? ctx.moveTo(p[0] * width, p[1] * height) : ctx.lineTo(p[0] * width, p[1] * height));
            ctx.stroke();
        }
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: canvas.page.regionSplit ? Qt.CrossCursor : Qt.PointingHandCursor
        onPressed: mouse => {
            if (canvas.page.regionSplit)
                canvas.add(mouse.x, mouse.y);
            else if (!canvas.previewing)
                canvas.toggle(canvas.pickAt(mouse.x, mouse.y));
        }
        onPositionChanged: mouse => {
            if (canvas.page.regionSplit && pressed)
                canvas.add(mouse.x, mouse.y);
            else if (canvas.previewing)
                canvas.aim(mouse.x, mouse.y);
        }
        onReleased: canvas.lay()
        onCanceled: canvas.lay()
    }
}
