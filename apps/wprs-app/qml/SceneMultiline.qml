import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Area {
    id: area

    required property var host
    readonly property string shown: String(host.value ?? "")

    Layout.fillWidth: true
    enterApplies: true
    QQC2.ToolTip.visible: hovered && activeFocus
    QQC2.ToolTip.text: App.tr("Enter applies the text; Shift+Enter starts a new line")
    text: shown
    wrapMode: TextEdit.Wrap
    selectByMouse: true
    Accessible.name: SceneWords.label(host.field.key)
    onEditingFinished: {
        if (text !== shown)
            host.commit(text);
    }
}
