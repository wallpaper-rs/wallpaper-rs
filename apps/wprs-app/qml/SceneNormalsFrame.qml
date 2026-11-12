import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: frame

    required property var page
    property alias seconds: at.value

    signal picked

    QQC2.Label {
        text: App.tr("Frame at %1 s", Math.round(at.value * 10) / 10)
    }

    SceneSlider {
        id: at

        editor: frame.page.editor
        Layout.fillWidth: true
        from: 0
        to: Math.max(10, Math.ceil(frame.page.editor.time))
        value: frame.page.editor.time
        onMoved: frame.picked()
    }

    QQC2.Button {
        text: App.tr("Playhead")
        onClicked: {
            at.value = frame.page.editor.time;
            frame.picked();
        }
    }
}
