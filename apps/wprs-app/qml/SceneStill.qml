import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: still

    required property var host
    property var outputs: App.state.outputs ?? {}
    property var names: App.outputs
    readonly property string screen: names.find(name => (still.outputs[name] ?? {}).item === still.host.uid) ?? ""
    readonly property var shown: outputs[screen] ?? {}
    readonly property bool heavy: screen !== "" && shown.quality_auto === "still"
    readonly property string reason: host.flashing ? "flash" : heavy ? "heavy" : ""

    function words(reason) {
        switch (reason) {
        case "flash":
            return App.tr("Still on its poster: it flashes, and flashing wallpapers are off in Settings › Playback.");
        case "heavy":
            return App.tr("Still on its poster: it is too heavy for this computer with the automatic quality. If you play it anyway it uses the lowest quality: it may drop frames and use more battery.");
        }
        return "";
    }

    function play() {
        App.act("SetOption", ["playback.quality." + screen, "min-q1"]);
    }

    visible: reason !== ""
    spacing: Look.e3

    Nota {
        Layout.fillWidth: true
        fitWidth: width
        text: still.words(still.reason)
        tone: "ambar"
    }

    QQC2.Button {
        visible: still.reason === "heavy"
        text: App.tr("Play anyway")
        onClicked: still.play()
    }
}
