import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: ring

    required property var host
    property var brush: null
    property var trail: []
    readonly property real span: Math.max(host.width, host.height)
    readonly property real diameter: brush ? 2 * brush.radius * span : 0
    readonly property var last: trail.length > 0 ? trail[trail.length - 1] : null
    readonly property real spotX: last ? last[0] * host.width : hover.point.position.x
    readonly property real spotY: last ? last[1] * host.height : hover.point.position.y

    visible: brush !== null && (hover.hovered || last !== null)
    onTrailChanged: stroke.requestPaint()
    onDiameterChanged: stroke.requestPaint()

    HoverHandler {
        id: hover

        target: null
    }

    Canvas {
        id: stroke

        anchors.fill: parent
        visible: ring.trail.length > 0
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            if (ring.trail.length === 0)
                return;
            ctx.strokeStyle = Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.3);
            ctx.lineWidth = ring.diameter;
            ctx.lineCap = "round";
            ctx.lineJoin = "round";
            ctx.beginPath();
            ring.trail.forEach((p, i) => {
                    if (i === 0)
                        ctx.moveTo(p[0] * width, p[1] * height);
                    else
                        ctx.lineTo(p[0] * width, p[1] * height);
                });
            if (ring.trail.length === 1)
                ctx.lineTo(ring.trail[0][0] * width + 0.01, ring.trail[0][1] * height);
            ctx.stroke();
        }
    }

    Rectangle {
        x: ring.spotX - width / 2
        y: ring.spotY - height / 2
        width: ring.diameter
        height: width
        radius: width / 2
        color: "transparent"
        border.width: 2
        border.color: Kirigami.Theme.highlightColor
    }
}
