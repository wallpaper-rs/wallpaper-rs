import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: row

    required property var owner
    required property var modelData

    Layout.fillWidth: true

    RowLayout {
        Layout.fillWidth: true

        Kirigami.Icon {
            source: "audio-x-generic"
            implicitWidth: Kirigami.Units.iconSizes.small
            implicitHeight: implicitWidth
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: row.modelData.id + (row.modelData.muted ? " (" + App.tr("muted") + ")" : "")
            elide: Text.ElideRight
            font.bold: true
        }

        QQC2.Label {
            text: owner.trigger(row.modelData)
            color: Kirigami.Theme.disabledTextColor
        }

        QQC2.ToolButton {
            checkable: true
            checked: owner.same(owner.heard, row.modelData.id)
            icon.name: checked ? "media-playback-stop" : "media-playback-start"
            text: checked ? App.tr("Stop listening") : App.tr("Listen")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: owner.listen(row.modelData.id)
        }

        QQC2.ToolButton {
            visible: owner.root
            icon.name: "go-next"
            text: App.tr("Edit the sound layer")
            display: QQC2.AbstractButton.IconOnly
            onClicked: owner.page.editor.select(row.modelData.pointer)
        }
    }

    Repeater {
        model: row.modelData.sounds

        delegate: ColumnLayout {
            id: clip

            required property int index
            required property string modelData

            Layout.fillWidth: true

            QQC2.Label {
                text: clip.modelData + (owner.wave(clip.modelData).seconds ? " · " + App.tr("%1 s", Number(owner.wave(clip.modelData).seconds).toFixed(1)) : "")
                color: Kirigami.Theme.disabledTextColor
            }

            SceneWave {
                Layout.fillWidth: true
                wave: owner.wave(clip.modelData)
                fadeIn: clip.index === 0 ? row.modelData.fade_in : 0
                fadeOut: clip.index === row.modelData.sounds.length - 1 ? row.modelData.fade_out : 0
                fades: clip.index === 0 || clip.index === row.modelData.sounds.length - 1
                onFaded: (key, seconds) => owner.page.run("set", {
                        "pointer": row.modelData.pointer + "/" + key,
                        "value": seconds
                    })
            }
        }
    }
}
