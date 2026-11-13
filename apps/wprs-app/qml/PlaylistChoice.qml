import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: choice

    property var playlists: []
    readonly property bool fresh: chooser.currentValue === ""
    readonly property bool ready: !fresh || name.text.trim() !== ""

    function resolve(done) {
        if (!choice.fresh)
            done(chooser.currentValue);
        else
            App.act("Create", [name.text.trim(), ""], "Playlists1", reply => done(reply[0]));
    }

    QQC2.ComboBox {
        id: chooser

        Layout.fillWidth: true
        model: [
            {
                "name": App.tr("A new playlist…"),
                "uid": ""
            }
        ].concat(choice.playlists)
        textRole: "name"
        valueRole: "uid"
        onCountChanged: currentIndex = count > 1 ? 1 : 0
    }

    QQC2.TextField {
        id: name

        Layout.fillWidth: true
        visible: choice.fresh
        placeholderText: App.tr("Name of the new playlist")
    }
}
