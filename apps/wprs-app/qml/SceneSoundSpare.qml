import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: loose

    required property var owner
    required property var modelData

    Layout.fillWidth: true

    RowLayout {
        Layout.fillWidth: true

        QQC2.Label {
            Layout.fillWidth: true
            text: loose.modelData.name
            elide: Text.ElideRight
        }

        QQC2.ToolButton {
            checkable: true
            checked: owner.same(owner.heard, {
                "asset": loose.modelData.name
            })
            icon.name: checked ? "media-playback-stop" : "media-playback-start"
            text: checked ? App.tr("Stop listening") : App.tr("Listen")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: owner.listen({
                "asset": loose.modelData.name
            })
        }
    }

    SceneWave {
        Layout.fillWidth: true
        wave: owner.wave(loose.modelData.name)
        fades: false
    }
}
