import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: widget

    property var stops: []
    property var live: null

    signal committed(var stops)

    function finish() {
        if (live === null)
            return;
        const done = live;
        live = null;
        committed(done);
    }

    SceneGradientBar {
        Layout.fillWidth: true
        stops: widget.live ?? widget.stops
        onEdited: (list, gesture) => {
            widget.live = list;
            if (gesture) {
                settle.restart();
            } else {
                settle.stop();
                widget.finish();
            }
        }
    }

    Timer {
        id: settle

        interval: 350
        onTriggered: widget.finish()
    }
}
