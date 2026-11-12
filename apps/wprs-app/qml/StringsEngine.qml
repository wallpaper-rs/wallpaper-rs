import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: engine

    property var texts: []

    Kirigami.Heading {
        Layout.fillWidth: true
        level: 5
        text: App.tr("Texts of the engine")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("The engine translates these for every user. Pick one with Use a text by key on a title or a label.")
    }

    Repeater {
        model: engine.texts

        delegate: QQC2.Label {
            required property var modelData

            Layout.fillWidth: true
            text: "@" + modelData.key + ": " + modelData.en + " / " + modelData.es
            elide: Text.ElideRight
        }
    }
}
