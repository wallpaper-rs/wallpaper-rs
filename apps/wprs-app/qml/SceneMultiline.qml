import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Area {
    id: area

    required property var host
    readonly property string shown: String(host.value ?? "")

    Layout.fillWidth: true
    text: shown
    wrapMode: TextEdit.Wrap
    selectByMouse: true
    Accessible.name: SceneWords.label(host.field.key)
    onEditingFinished: {
        if (text !== shown)
            host.commit(text);
    }
}
