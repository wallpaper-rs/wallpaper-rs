import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: curve

    required property var tool
    readonly property var current: tool ? tool.current : null
    readonly property var shape: current ? current.shape : []
    readonly property var points: current && Array.isArray(current.curve) ? current.curve : [0.33, 0, 0.67, 1]
    readonly property real progress: tool ? tool.progress : 0
    readonly property real pad: Look.e3
    readonly property real wide: width - pad * 2
    readonly property real tall: height - pad * 2

    function sx(x) {
        return pad + x * wide;
    }

    function sy(y) {
        return pad + (1 - y) * tall;
    }

    function at(t) {
        if (shape.length < 2)
            return t;
        const i = Math.min(shape.length - 2, Math.floor(t * (shape.length - 1)));
        const f = t * (shape.length - 1) - i;
        return shape[i] + (shape[i + 1] - shape[i]) * f;
    }

    function put(which, x, y) {
        const p = points.slice();
        p[which * 2] = Math.round(Math.max(0, Math.min(1, x)) * 100) / 100;
        p[which * 2 + 1] = Math.round(Math.max(0, Math.min(1, y)) * 100) / 100;
        tool.act({
            "curve": p
        });
    }

    onShapeChanged: paper.requestPaint()
    onProgressChanged: paper.requestPaint()
    onWidthChanged: paper.requestPaint()
    onHeightChanged: paper.requestPaint()

    Canvas {
        id: paper

        anchors.fill: parent
        onPaint: {
            const g = getContext("2d");
            g.reset();
            g.strokeStyle = Look.nocheLinea;
            g.lineWidth = 1;
            g.strokeRect(curve.pad, curve.pad, curve.wide, curve.tall);
            g.strokeStyle = Look.bruma;
            g.beginPath();
            g.moveTo(curve.sx(0), curve.sy(0));
            g.lineTo(curve.sx(curve.points[0]), curve.sy(curve.points[1]));
            g.moveTo(curve.sx(1), curve.sy(1));
            g.lineTo(curve.sx(curve.points[2]), curve.sy(curve.points[3]));
            g.stroke();
            g.strokeStyle = Look.luna;
            g.lineWidth = 2;
            g.beginPath();
            for (let i = 0; i < curve.shape.length; i++) {
                const t = i / (curve.shape.length - 1);
                if (i === 0)
                    g.moveTo(curve.sx(t), curve.sy(curve.shape[i]));
                else
                    g.lineTo(curve.sx(t), curve.sy(curve.shape[i]));
            }
            g.stroke();
            g.strokeStyle = Look.ambar;
            g.lineWidth = 1;
            g.beginPath();
            g.moveTo(curve.sx(curve.progress), curve.pad);
            g.lineTo(curve.sx(curve.progress), curve.pad + curve.tall);
            g.stroke();
            g.fillStyle = Look.ambar;
            g.beginPath();
            g.arc(curve.sx(curve.progress), curve.sy(curve.at(curve.progress)), 4, 0, Math.PI * 2);
            g.fill();
        }
    }

    Repeater {
        model: 2

        delegate: Rectangle {
            id: knob

            required property int index

            x: curve.sx(curve.points[index * 2]) - width / 2
            y: curve.sy(curve.points[index * 2 + 1]) - height / 2
            width: Look.rielPunto
            height: Look.rielPunto
            radius: width / 2
            color: grab.drag.active ? Look.ambar : Look.nocheAlta
            border.color: activeFocus ? Look.escarcha : Look.luna
            border.width: activeFocus ? Look.linea * 3 : Look.linea * 2
            activeFocusOnTab: true
            Accessible.role: Accessible.Slider
            Accessible.name: index === 0 ? App.tr("Curve start handle") : App.tr("Curve end handle")
            Keys.onLeftPressed: curve.put(index, curve.points[index * 2] - 0.02, curve.points[index * 2 + 1])
            Keys.onRightPressed: curve.put(index, curve.points[index * 2] + 0.02, curve.points[index * 2 + 1])
            Keys.onUpPressed: curve.put(index, curve.points[index * 2], curve.points[index * 2 + 1] + 0.02)
            Keys.onDownPressed: curve.put(index, curve.points[index * 2], curve.points[index * 2 + 1] - 0.02)

            MouseArea {
                id: grab

                anchors.fill: parent
                cursorShape: Qt.SizeAllCursor
                drag.target: knob
                onPressed: knob.forceActiveFocus()
                onReleased: curve.put(knob.index, (knob.x + knob.width / 2 - curve.pad) / curve.wide, 1 - (knob.y + knob.height / 2 - curve.pad) / curve.tall)
            }
        }
    }
}
