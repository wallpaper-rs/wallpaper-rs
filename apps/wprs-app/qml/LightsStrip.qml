import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: lights

    property string output
    property bool active: false
    property var strip: null

    onOutputChanged: strip = null

    Timer {
        interval: 100
        repeat: true
        running: lights.active && lights.output !== ""
        onTriggered: {
            const asked = lights.output;
            App.query("GetStrip", [asked], "Rgb1", reply => lights && lights.output === asked && (lights.strip = reply), () => lights && (lights.strip = null));
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: lights.strip ? App.tr("What the lights get, %1 × %2:", lights.strip[0], lights.strip[1]) : App.tr("The page has sent no colors to the lights yet; pages that call window.wprs.rgb.setImage show them here.")
    }

    Grid {
        visible: lights.strip !== null
        columns: Math.max(1, lights.strip?.[0] ?? 1)
        spacing: 1

        Repeater {
            model: lights.strip ? lights.strip[0] * lights.strip[1] : 0

            delegate: Rectangle {
                required property int index

                width: Kirigami.Units.gridUnit * 0.4
                height: width
                color: Qt.rgba(lights.strip[2][index * 3] / 255, lights.strip[2][index * 3 + 1] / 255, lights.strip[2][index * 3 + 2] / 255, 1)
            }
        }
    }
}
