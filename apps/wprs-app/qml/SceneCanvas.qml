import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: canvas

    required property var page
    readonly property var size: page.view.canvas?.size ?? [1920, 1080]
    readonly property real fitted: Math.min(area.width / size[0], area.height / size[1]) * 0.94
    readonly property real ratio: fitted * zoom
    property real zoom: 1
    property point pan: Qt.point(0, 0)
    property var stream: null
    property bool missing: false
    property bool testing: false
    readonly property var aspects: ["", "16:9", "21:9", "32:9", "4:3", "16:10", "9:16"]

    function connect() {
        stream?.destroy();
        stream = null;
        if (App.checking || page.editor.node === 0)
            return;
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { anchors.fill: parent }", screen);
            if (page.editor.serial > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = page.editor.serial;
            else
                stream.nodeId = page.editor.node;
        } catch (error) {
            missing = true;
        }
    }

    spacing: 0

    Connections {
        target: canvas.page.editor

        function onSerialChanged() {
            canvas.connect();
        }
    }

    SceneCanvasBar {
        id: bar

        Layout.fillWidth: true
        canvas: canvas
    }

    Rectangle {
        id: area

        Layout.fillWidth: true
        Layout.fillHeight: true
        color: Look.noche
        clip: true

        DragHandler {
            property point start

            target: null
            acceptedButtons: Qt.MiddleButton | Qt.RightButton
            onActiveChanged: start = canvas.pan
            onTranslationChanged: canvas.pan = Qt.point(start.x + translation.x, start.y + translation.y)
        }

        WheelHandler {
            onWheel: wheel => {
                if (wheel.modifiers & Qt.ControlModifier)
                    canvas.zoom = Math.max(0.1, Math.min(16, canvas.zoom * (wheel.angleDelta.y > 0 ? 1.1 : 1 / 1.1)));
                else
                    canvas.pan = Qt.point(canvas.pan.x + wheel.angleDelta.x / 4, canvas.pan.y + wheel.angleDelta.y / 4);
            }
        }

        TapHandler {
            onTapped: {
                canvas.page.picking = "";
                canvas.page.editor.select("/scene/canvas");
                canvas.SceneFocus.canvasWanted();
            }
        }

        PointHandler {
            acceptedButtons: Qt.LeftButton
            onActiveChanged: canvas.page.editor.hold(active)
        }

        SceneDropFiles {
            anchors.fill: parent
            z: 6
            layers: true
            page: canvas.page
        }

        SceneBand {
            anchors.fill: parent
            enabled: !canvas.page.moving && !canvas.testing
            page: canvas.page
            screen: screen
            ratio: canvas.ratio
        }

        Rectangle {
            id: screen

            x: (area.width - width) / 2 + canvas.pan.x
            y: (area.height - height) / 2 + canvas.pan.y
            width: canvas.size[0] * canvas.ratio
            height: canvas.size[1] * canvas.ratio
            color: canvas.page.view.canvas?.background ?? "black"
            border.color: Look.nocheLinea

            QQC2.Label {
                anchors.centerIn: parent
                width: parent.width * 0.8
                visible: canvas.stream === null
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.Wrap
                color: Look.escarcha
                text: canvas.missing ? App.tr("Install the KPipeWire QML module to see the preview here, or use Show on the desktop.") : App.tr("The preview appears here while wprsd runs.")
            }

            SceneOverlays {
                anchors.fill: parent
                z: 4
                page: canvas.page
                ratio: canvas.ratio
            }

            SceneSpan {
                anchors.fill: parent
                z: 5
                page: canvas.page
                ratio: canvas.ratio
                layout: bar.spans.layout
            }

            SceneGuide {
                anchors.centerIn: parent
                z: 3
                size: canvas.size
                ratio: canvas.ratio
                aspect: canvas.aspects[bar.guides.currentIndex]
            }

            SceneCanvasTouch {
                anchors.fill: parent
                page: canvas.page
                testing: canvas.testing
            }

            SceneTransitionView {
                anchors.fill: parent
                z: 6
                page: canvas.page
            }
        }
    }
}
