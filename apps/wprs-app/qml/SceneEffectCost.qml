import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: cost

    required property var page
    readonly property string effect: cost.page.fields.find(f => f.key === "effect")?.value ?? ""
    readonly property bool optional: cost.page.fields.find(f => f.key === "optional")?.value ?? false
    property var info: null

    function load() {
        cost.info = cost.effect ? JSON.parse(cost.page.editor.rig("", JSON.stringify({
                        "effect_cost": {
                            "effect": cost.effect,
                            "optional": cost.optional
                        }
                    })) || "null") : null;
    }

    onEffectChanged: load()
    onOptionalChanged: load()
    Component.onCompleted: load()

    Kirigami.Heading {
        level: 5
        text: App.tr("Cost")
    }

    SceneQuality {
        page: cost.page
    }

    SceneRepeater {
        id: listed

        rows: cost.info?.levels ?? []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            font.bold: modelData.quality === SceneLook.effective
            text: modelData.units === null ? App.tr("%1: left out of the scene at this level", modelData.quality.toUpperCase()) : App.tr("%1: %2 units of work", modelData.quality.toUpperCase(), modelData.units.toFixed(2))
        }
    }
}
