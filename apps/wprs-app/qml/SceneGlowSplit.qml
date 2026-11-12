import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: glow

    required property var page
    property var stream: null
    property real at: 0.5
    readonly property var half: JSON.parse(page.editor.split || "{}")
    readonly property bool shown: stream !== null && (half.on ?? false)

    function connect() {
        stream?.destroy();
        stream = null;
        if (App.checking || !half.node)
            return;
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { x: -cut.x; width: glow.width; height: glow.height }", cut);
            if (half.serial > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = half.serial;
            else
                stream.nodeId = half.node;
        } catch (error) {
            stream = null;
        }
    }

    function drag(x) {
        glow.at = Math.max(0.05, Math.min(0.95, x / Math.max(1, glow.width)));
    }

    onHalfChanged: connect()
    Component.onCompleted: connect()

    Item {
        id: cut

        x: glow.width * glow.at
        width: glow.width - x
        height: glow.height
        visible: glow.shown
        clip: true
    }
}
