import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: texts

    required property var host
    readonly property var field: host.field
    readonly property bool multiline: field.widget === "markup"
    readonly property bool lines: field.pointer.startsWith("/scene/")
    readonly property bool last: (field.key === "title" || lines) && field.options.length < 2

    function put(language, text) {
        host.page.run("translate", {
                "pointer": field.pointer,
                "language": language,
                "text": text
            });
    }

    SceneRepeater {
        id: listed

        rows: texts.field.options

        delegate: RowLayout {
            id: row

            required property int index
            readonly property var modelData: listed.rows[index]
            readonly property string text: String(modelData.text ?? "")

            Layout.fillWidth: true

            QQC2.Label {
                visible: row.modelData.language !== ""
                Layout.alignment: Qt.AlignTop
                Layout.topMargin: Kirigami.Units.smallSpacing
                text: ManifestWords.language(row.modelData.language)
            }

            Area {
                Layout.fillWidth: true
                visible: texts.lines
                enterApplies: true
                text: row.text
                wrapMode: TextEdit.Wrap
                selectByMouse: true
                Accessible.name: SceneWords.label(texts.field.key) + " " + ManifestWords.language(row.modelData.language)
                onEditingFinished: {
                    if (text !== row.text)
                        texts.put(row.modelData.language, text);
                }
            }

            QQC2.TextField {
                Layout.fillWidth: true
                visible: !texts.multiline && !texts.lines
                text: row.text
                Accessible.name: SceneWords.label(texts.field.key) + " " + ManifestWords.language(row.modelData.language)
                onEditingFinished: {
                    if (text !== row.text)
                        texts.put(row.modelData.language, text);
                }
            }

            MarkupField {
                Layout.fillWidth: true
                visible: texts.multiline && !texts.lines
                text: row.text
                label: SceneWords.label(texts.field.key) + " " + ManifestWords.language(row.modelData.language)
                onCommitted: value => {
                    if (value !== row.text)
                        texts.put(row.modelData.language, value);
                }
            }

            QQC2.ToolButton {
                visible: row.modelData.language !== "" && !texts.last
                Layout.alignment: Qt.AlignTop
                icon.name: "edit-delete"
                text: App.tr("Remove this language")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: texts.put(row.modelData.language, null)
            }
        }
    }

    SceneTranslate {
        host: texts.host
        keys: !texts.lines
    }
}
