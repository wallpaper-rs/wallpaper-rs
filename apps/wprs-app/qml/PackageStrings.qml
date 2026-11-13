import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: strings

    required property var host
    readonly property var page: host.page
    readonly property var info: {
        page.view;
        return JSON.parse(page.editor.rig("", JSON.stringify({
                        "strings": {}
                    })) || "{}");
    }
    readonly property var languages: info.languages ?? []
    readonly property var keys: info.keys ?? []
    readonly property var lacking: keys.reduce((count, k) => count + k.missing.length, 0)

    function put(key, language, text) {
        page.run("manifest", {
                "text": {
                    "key": key,
                    "language": language,
                    "text": text === "" ? null : text
                }
            });
    }

    function drop(key) {
        const row = keys.find(k => k.key === key);
        Object.keys(row?.texts ?? {}).forEach(language => put(key, language, ""));
    }

    spacing: Kirigami.Units.smallSpacing

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: strings.keys.length === 0 ? App.tr("No texts by key yet. A text that starts with @ takes its words from this table, so one key can be translated once and used in many places.") : strings.lacking > 0 ? App.tr("%1 texts are missing in some language.", strings.lacking) : App.tr("Every key has a text in every language.")
    }

    Repeater {
        model: strings.languages

        delegate: RowLayout {
            id: language

            required property string modelData
            readonly property bool filed: strings.info.places[modelData] === "file"

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                text: ManifestWords.language(language.modelData)
            }

            QQC2.Switch {
                text: App.tr("In its own file")
                visible: strings.info.places[language.modelData] !== undefined
                checked: language.filed
                onToggled: strings.page.run("manifest", {
                        "locale": {
                            "language": language.modelData,
                            "file": checked
                        }
                    })
                QQC2.ToolTip.text: App.tr("Keep the texts of this language in locale/%1.json instead of the manifest", language.modelData)
                QQC2.ToolTip.visible: hovered
            }
        }
    }

    Repeater {
        model: strings.keys

        delegate: ColumnLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true
            spacing: 0

            RowLayout {
                Layout.fillWidth: true

                QQC2.Label {
                    Layout.fillWidth: true
                    font.bold: true
                    text: "@" + row.modelData.key + (row.modelData.used ? "" : " · " + App.tr("not used"))
                    elide: Text.ElideRight
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    text: App.tr("Remove this key")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: strings.drop(row.modelData.key)
                }
            }

            Repeater {
                model: strings.languages

                delegate: RowLayout {
                    id: cell

                    required property string modelData
                    readonly property string text: row.modelData.texts[modelData] ?? ""

                    Layout.fillWidth: true

                    QQC2.Label {
                        text: cell.modelData
                        color: cell.text === "" ? Look.coral : Kirigami.Theme.textColor
                    }

                    QQC2.TextField {
                        Layout.fillWidth: true
                        text: cell.text
                        placeholderText: App.tr("Missing")
                        Accessible.name: row.modelData.key + " " + cell.modelData
                        onEditingFinished: {
                            if (text !== cell.text)
                                strings.put(row.modelData.key, cell.modelData, text);
                        }
                    }
                }
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            id: key

            Layout.fillWidth: true
            placeholderText: App.tr("New key, such as greeting")
            Accessible.name: placeholderText
        }

        QQC2.ComboBox {
            id: into

            model: strings.languages
        }

        QQC2.TextField {
            id: words

            Layout.fillWidth: true
            placeholderText: App.tr("Its text")
            Accessible.name: placeholderText
        }

        QQC2.Button {
            icon.name: "list-add"
            text: App.tr("Add")
            enabled: /^[A-Za-z0-9_.-]{1,64}$/.test(key.text) && !key.text.startsWith("wprs.") && words.text.trim() !== "" && strings.languages.length > 0
            onClicked: {
                strings.put(key.text, into.currentText, words.text);
                if (strings.page.editor.error === "") {
                    key.clear();
                    words.clear();
                }
            }
        }
    }

    StringsEngine {
        Layout.fillWidth: true
        texts: strings.info.engine ?? []
    }
}
