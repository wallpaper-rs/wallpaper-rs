import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: brush

    required property var stage
    readonly property var page: stage.page
    readonly property var tool: page.brush
    readonly property bool combing: tool?.action === "comb3d"
    readonly property bool keying: tool?.action === "key3d"
    property var stroke: []
    property var hover: null

    function at(mouse) {
        return [Math.round(mouse.x / stage.ratio * 10) / 10, Math.round(mouse.y / stage.ratio * 10) / 10];
    }

    function lay() {
        const points = stroke.slice(0, 2000);
        stroke = [];
        line.requestPaint();
        if (!combing || points.length < 2)
            return;
        page.run("scene3d", {
                "comb": {
                    "layer": tool.layer,
                    "name": tool.name,
                    "points": points,
                    "radius": tool.radius,
                    "strength": tool.strength,
                    "hardness": tool.hardness,
                    "erase": tool.erase,
                    "time": page.editor.time
                }
            });
    }

    visible: combing || keying

    Canvas {
        id: line

        anchors.fill: parent
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            const r = (brush.tool?.radius ?? 0) * brush.stage.ratio;
            ctx.strokeStyle = brush.tool?.erase ? Kirigami.Theme.negativeTextColor : Kirigami.Theme.highlightColor;
            ctx.lineWidth = 2;
            if (brush.combing && brush.hover) {
                ctx.beginPath();
                ctx.arc(brush.hover[0] * brush.stage.ratio, brush.hover[1] * brush.stage.ratio, r, 0, 2 * Math.PI);
                ctx.stroke();
            }
            ctx.beginPath();
            brush.stroke.forEach((p, i) => i === 0 ? ctx.moveTo(p[0] * brush.stage.ratio, p[1] * brush.stage.ratio) : ctx.lineTo(p[0] * brush.stage.ratio, p[1] * brush.stage.ratio));
            ctx.stroke();
        }
    }

    MouseArea {
        anchors.fill: parent
        enabled: brush.visible
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton
        cursorShape: Qt.CrossCursor
        onPressed: mouse => {
            brush.stroke = [brush.at(mouse)];
            line.requestPaint();
        }
        onPositionChanged: mouse => {
            const p = brush.at(mouse);
            brush.hover = p;
            const last = brush.stroke[brush.stroke.length - 1];
            if (pressed && brush.combing && Math.hypot(p[0] - last[0], p[1] - last[1]) > Math.max(1, brush.tool.radius / 4))
                brush.stroke = brush.stroke.concat([p]);
            line.requestPaint();
        }
        onExited: {
            brush.hover = null;
            line.requestPaint();
        }
        onReleased: mouse => {
            if (brush.keying) {
                brush.page.run("scene3d", {
                        "key": {
                            "layer": brush.tool.layer,
                            "name": brush.tool.name,
                            "at": brush.at(mouse),
                            "time": brush.page.editor.time
                        }
                    });
                brush.stroke = [];
                brush.page.brush = null;
                return;
            }
            brush.lay();
        }
    }

    QQC2.Label {
        x: Kirigami.Units.smallSpacing
        y: Kirigami.Units.smallSpacing
        text: brush.keying ? App.tr("Click the model where it shows the color to key out") : App.tr("Drag over the fur to comb it; each stroke can be undone")
        color: Kirigami.Theme.highlightColor
    }
}
