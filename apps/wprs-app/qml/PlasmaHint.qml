import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2

QQC2.Label {
    required property var host

    Layout.fillWidth: true
    visible: host.daemon.state?.plasma === true && host.info.blank === true
    text: host.daemon.tr("Plasma shows the background color here. To use another Plasma wallpaper type, press «Use Plasma's own wallpaper» or choose it in the desktop settings.")
    wrapMode: Text.Wrap
}
