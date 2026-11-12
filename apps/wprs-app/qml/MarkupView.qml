import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: view

    property string source
    readonly property int pad: Kirigami.Units.smallSpacing * 2

    implicitHeight: shown.implicitHeight + 2 * pad
    color: Kirigami.Theme.backgroundColor
    border.width: 1
    border.color: Kirigami.Theme.disabledTextColor
    Accessible.role: Accessible.StaticText
    Accessible.name: App.plain(source)

    QQC2.Label {
        id: shown

        x: view.pad
        y: view.pad
        width: view.width - 2 * view.pad
        text: view.source.trim() === "" ? App.tr("Nothing to show yet") : App.markup(view.source)
        textFormat: view.source.trim() === "" ? Text.PlainText : Text.StyledText
        color: view.source.trim() === "" ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor
        wrapMode: Text.Wrap
    }
}
