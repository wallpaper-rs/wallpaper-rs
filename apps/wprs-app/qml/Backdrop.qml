import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: backdrop

    property url source
    property bool held: false
    property real framing: 0.7
    readonly property bool still: Kirigami.Units.longDuration <= 1
    property Image front: one
    readonly property Image back: front === one ? two : one

    function load() {
        if (front.source === source)
            return;
        back.opacity = 0;
        back.source = source;
    }

    function reveal(layer) {
        if (layer !== back || layer.status !== Image.Ready)
            return;
        fade.stop();
        layer.z = 1;
        front.z = 0;
        if (still) {
            const old = front;
            layer.opacity = 1;
            front = layer;
            old.opacity = 0;
            old.source = "";
            return;
        }
        fade.target = layer;
        fade.start();
    }

    clip: true
    opacity: held ? 0.6 : 1
    onSourceChanged: load()
    Component.onCompleted: load()

    component Layer: Image {
        readonly property real fit: implicitWidth > 0 ? Math.max(backdrop.width / implicitWidth, backdrop.height / implicitHeight) : 1

        width: implicitWidth * fit
        height: implicitHeight * fit
        x: (backdrop.width - width) * backdrop.framing
        y: (backdrop.height - height) / 2
        asynchronous: true
        cache: false
        fillMode: Image.Stretch
        opacity: 0
        onStatusChanged: backdrop.reveal(this)
    }

    Layer {
        id: one
    }

    Layer {
        id: two
    }

    NumberAnimation {
        id: fade

        property: "opacity"
        from: 0
        to: 1
        duration: Kirigami.Units.longDuration
        easing.type: Easing.BezierSpline
        easing.bezierCurve: Look.pasoCurva
        onFinished: {
            const old = backdrop.front;
            backdrop.front = fade.target;
            old.opacity = 0;
            old.source = "";
        }
    }
}
