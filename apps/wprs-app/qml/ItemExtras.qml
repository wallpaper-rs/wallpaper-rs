import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: extras

    required property var host

    Kirigami.Heading {
        level: 4
        text: App.tr("Tags")
    }

    Flow {
        Layout.fillWidth: true
        spacing: Kirigami.Units.smallSpacing

        Repeater {
            model: extras.host.item.tags ?? []

            delegate: Kirigami.Chip {
                required property string modelData

                text: Labels.tag(modelData)
                closable: true
                checkable: false
                onRemoved: App.act("SetTags", [[extras.host.uid], [], [modelData]], "Library1")
            }
        }

        QQC2.TextField {
            placeholderText: App.tr("Add a tag")
            onAccepted: {
                if (text.trim())
                    App.act("SetTags", [[extras.host.uid], [text.trim()], []], "Library1");
                clear();
            }
        }
    }

    Kirigami.Heading {
        level: 4
        visible: playlists.count > 0
        text: App.tr("In playlists")
    }

    Repeater {
        id: playlists

        model: extras.host.details.playlists ?? []

        delegate: RowLayout {
            required property var modelData

            QQC2.Label {
                Layout.fillWidth: true
                text: modelData.name
            }

            QQC2.ToolButton {
                icon.name: "list-remove"
                text: App.tr("Remove from this playlist")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: App.act("RemoveItems", [modelData.uid, [extras.host.uid]], "Playlists1")
            }
        }
    }
}
