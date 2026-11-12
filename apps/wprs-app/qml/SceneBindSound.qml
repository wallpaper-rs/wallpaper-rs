import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    signal picked(var request)
    signal ranged

    title: App.tr("Follow the sound")

    Repeater {
        model: [["bass", App.tr("Bass")], ["mid", App.tr("Mids")], ["treble", App.tr("Treble")], ["level", App.tr("Level")], ["beat", App.tr("Beat")]]

        delegate: QQC2.MenuItem {
            required property var modelData

            text: modelData[1]
            onTriggered: picked({
                    "pick": modelData[0]
                })
        }
    }

    QQC2.MenuSeparator {}

    QQC2.MenuItem {
        text: App.tr("A band or a range of bands…")
        onTriggered: ranged()
    }
}
