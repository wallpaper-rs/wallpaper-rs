import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: settings

    required property var page
    required property var catalog

    function put(id, value, gesture) {
        const values = {};
        values[id] = value;
        page.manifestEdit("preset", {
                "values": values
            }, gesture === true);
    }

    function clear(id) {
        page.manifestEdit("preset", {
                "clear": [id]
            });
    }

    function shown(field) {
        return field.set ? field.value : field.default;
    }

    function text(field) {
        const value = shown(field);
        return typeof value === "number" ? Number(value.toFixed(3)).toLocaleString(Qt.locale()) : String(value ?? "");
    }

    spacing: Kirigami.Units.smallSpacing

    Repeater {
        model: settings.catalog?.fields ?? []

        delegate: RowLayout {
            id: row

            required property var modelData
            readonly property string widget: modelData.widget
            readonly property var current: settings.shown(modelData)

            Layout.fillWidth: true

            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 9
                elide: Text.ElideRight
                textFormat: Text.PlainText
                font.bold: row.modelData.set
                text: row.modelData.title
            }

            SceneSlider {
                Layout.fillWidth: true
                visible: row.widget === "number" || row.widget === "integer"
                editor: settings.page.editor
                from: row.modelData.range[0] ?? 0
                to: row.modelData.range[1] ?? 1
                stepSize: row.widget === "integer" ? 1 : 0
                value: Number(row.current)
                Accessible.name: row.modelData.title
                onMoved: settings.put(row.modelData.key, row.widget === "integer" ? Math.round(value) : Math.round(value * 1000) / 1000, true)
            }

            QQC2.Label {
                visible: row.widget === "number" || row.widget === "integer"
                text: settings.text(row.modelData) + (row.modelData.unit ? " " + row.modelData.unit : "")
            }

            SceneColor {
                visible: row.widget === "rgb" || row.widget === "color"
                alpha: row.widget === "color"
                hex: typeof row.current === "string" ? row.current : "#ffffff"
                onPicked: hex => settings.put(row.modelData.key, hex)
            }

            QQC2.Switch {
                visible: row.widget === "bool"
                checked: row.current === true
                Accessible.name: row.modelData.title
                onToggled: settings.put(row.modelData.key, checked)
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                visible: row.widget === "ref:option"
                model: row.modelData.names ?? []
                currentIndex: (row.modelData.options ?? []).indexOf(row.current)
                Accessible.name: row.modelData.title
                onActivated: index => settings.put(row.modelData.key, row.modelData.options[index])
            }

            QQC2.TextField {
                Layout.fillWidth: true
                visible: row.widget === "text"
                text: String(row.current ?? "")
                Accessible.name: row.modelData.title
                onEditingFinished: if (text !== String(row.current ?? ""))
                    settings.put(row.modelData.key, text)
            }

            QQC2.ToolButton {
                icon.name: "edit-reset"
                text: App.tr("Default")
                display: QQC2.AbstractButton.IconOnly
                enabled: row.modelData.set
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: settings.clear(row.modelData.key)
            }
        }
    }

    Repeater {
        model: settings.catalog?.files ?? []

        delegate: RowLayout {
            id: media

            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 9
                elide: Text.ElideRight
                textFormat: Text.PlainText
                font.bold: media.modelData.file !== null
                text: media.modelData.title
            }

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                textFormat: Text.PlainText
                color: Kirigami.Theme.disabledTextColor
                text: media.modelData.file ?? App.tr("No file")
            }

            QQC2.Button {
                text: App.tr("Picture or video…")
                icon.name: "document-open"
                onClicked: chooser.open()
            }

            QQC2.ToolButton {
                icon.name: "edit-reset"
                text: App.tr("Default")
                display: QQC2.AbstractButton.IconOnly
                enabled: media.modelData.file !== null
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: settings.clear(media.modelData.id)
            }

            FileDialog {
                id: chooser

                nameFilters: [App.tr("Pictures and videos (*.png *.jpg *.jpeg *.webp *.gif *.webm *.mp4 *.mkv)")]
                onAccepted: {
                    const files = {};
                    files[media.modelData.id] = App.path(selectedFile);
                    settings.page.manifestEdit("preset", {
                            "files": files
                        });
                }
            }
        }
    }
}
