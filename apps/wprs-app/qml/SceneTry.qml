import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ScrollView {
    id: tryout

    required property var page
    readonly property var form: tryout.ask(tryout.page.manifest, SceneLook.values, SceneLook.preset)
    readonly property var fields: (form.fields ?? []).filter(field => field.visible)
    readonly property var presets: form.presets ?? []
    readonly property bool trying: SceneLook.values !== null || SceneLook.preset !== ""

    function ask(manifest, values, preset) {
        const language = Qt.locale().name.replace("_", "-");
        return JSON.parse(tryout.page.editor.rig("", JSON.stringify({
                "tried": {
                    "preset": preset,
                    "values": values ?? {},
                    "languages": [language, language.split("-")[0]]
                }
            })) || "{}");
    }

    contentWidth: availableWidth

    ColumnLayout {
        width: tryout.availableWidth

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Try the settings of the package in the preview. Only the preview shows them: nothing is saved and no screen changes.")
        }

        RowLayout {
            Layout.fillWidth: true
            visible: tryout.presets.length > 0

            QQC2.Label {
                text: App.tr("Preset")
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                textRole: "label"
                valueRole: "value"
                model: [{
                        "label": App.tr("Package defaults"),
                        "value": ""
                    }].concat(tryout.presets)
                currentIndex: Math.max(0, 1 + tryout.presets.findIndex(p => p.value === SceneLook.preset))
                onActivated: index => index === 0 ? SceneLook.untry(tryout.page) : SceneLook.tryPreset(tryout.page, tryout.presets[index - 1].value)
            }
        }

        QQC2.Button {
            icon.name: "edit-reset"
            text: App.tr("Back to the defaults")
            enabled: tryout.trying
            onClicked: SceneLook.untry(tryout.page)
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: tryout.fields.length === 0
            wrapMode: Text.Wrap
            color: Kirigami.Theme.disabledTextColor
            text: App.tr("The package declares no properties to try.")
        }

        SceneRepeater {
            id: listed

            rows: tryout.fields

            delegate: SceneField {
                required property int index
                modelData: listed.rows[index]

                Layout.fillWidth: true
                daemon: App
                output: ""
                onEdited: (key, value) => SceneLook.tryValue(tryout.page, key, value)
            }
        }
    }
}
