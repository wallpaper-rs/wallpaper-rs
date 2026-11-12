import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ScrollView {
    id: presets

    required property var page
    readonly property var rows: page.view.presets ?? []

    function add() {
        const at = rows.length;
        page.run("add_preset");
        if (rows.length > at)
            page.editor.select("/manifest/presets/" + at);
    }

    contentWidth: availableWidth
    clip: true

    ColumnLayout {
        width: presets.availableWidth
        spacing: Kirigami.Units.smallSpacing

        QQC2.Label {
            Layout.fillWidth: true
            Layout.margins: Kirigami.Units.largeSpacing
            wrapMode: Text.Wrap
            color: Kirigami.Theme.disabledTextColor
            text: App.tr("A preset sets the speed, the part that plays, the volume, the zoom and the part shown in one click; pick one to set its values on the right.")
        }

        SceneRepeater {
            id: listed

            rows: presets.rows

            delegate: QQC2.ItemDelegate {
                id: preset

                required property int index
                readonly property var modelData: listed.rows[index]

                Layout.fillWidth: true
                highlighted: presets.page.editor.selected === modelData.pointer
                icon.name: "bookmarks"
                text: modelData.label
                onClicked: presets.page.editor.select(modelData.pointer)

                QQC2.ToolButton {
                    anchors.right: parent.right
                    icon.name: "edit-delete"
                    text: App.tr("Remove")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: {
                        presets.page.editor.select("/manifest");
                        presets.page.run("remove", {
                                "pointer": preset.modelData.pointer
                            });
                    }
                }
            }
        }

        QQC2.ToolButton {
            text: App.tr("Add preset")
            icon.name: "list-add"
            onClicked: presets.add()
        }
    }
}
