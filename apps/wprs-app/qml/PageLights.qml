import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: trial

    property string uid
    property string output
    property var stream: null
    property bool missing: false
    property string problem

    function start(uid) {
        stop();
        trial.uid = uid;
        output = "preview:page-lights";
        problem = "";
        missing = false;
        open();
        const asked = output;
        App.query("RequestStream", [asked, 640, 360], "Daemon1", reply => trial && trial.output === asked && trial.attach(reply), error => trial && (trial.problem = error.message));
    }

    function attach(reply) {
        const found = typeof reply === "string" ? JSON.parse(reply) : reply;
        App.act("Apply", [uid, output, "", ""], "Library1", () => {}, error => trial && (trial.problem = error.message));
        if (App.checking)
            return;
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { anchors.fill: parent }", screen);
            if (found[1] > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = found[1];
            else
                stream.nodeId = found[0];
        } catch (error) {
            missing = true;
        }
    }

    function stop() {
        stream?.destroy();
        stream = null;
        if (output !== "")
            App.act("ReleaseStream", [output], "Daemon1");
        output = "";
    }

    title: App.tr("The page and its lights")
    preferredWidth: Kirigami.Units.gridUnit * 30
    standardButtons: Kirigami.Dialog.Close
    onClosed: stop()

    ColumnLayout {
        Rectangle {
            id: screen

            Layout.fillWidth: true
            Layout.preferredHeight: width * 9 / 16
            color: "black"

            QQC2.Label {
                anchors.centerIn: parent
                visible: trial.missing
                color: "white"
                text: App.tr("Install the KPipeWire QML module to see the page here.")
            }
        }

        LightsStrip {
            Layout.fillWidth: true
            output: trial.output
            active: trial.opened
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: trial.problem ? Kirigami.Theme.negativeTextColor : Kirigami.Theme.disabledTextColor
            text: trial.problem || App.tr("Here window.wprs.rgb.setImage returns true and its colors are only shown, never sent. On the desktop it returns false and sends nothing until RGB lighting is on and you allow it for this package.")
        }
    }
}
