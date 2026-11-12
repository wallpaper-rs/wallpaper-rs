import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: levels

    required property var page
    readonly property string property: levels.page.fields.find(f => f.key === "quality" && f.widget === "group")?.pointer.split("/")[3]?.replace(/~1/g, "/").replace(/~0/g, "~") ?? ""
    property var info: null

    function load() {
        info = property ? JSON.parse(page.editor.rig("", JSON.stringify({
                        "levels": {
                            "property": property
                        }
                    })) || "null") : null;
    }

    function shown(value) {
        return typeof value === "number" ? Number(value.toFixed(3)).toLocaleString(Qt.locale()) : String(value);
    }

    visible: property !== ""
    onPropertyChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            levels.load();
        }

        target: levels.page.editor
    }

    Kirigami.Heading {
        level: 5
        text: App.tr("Value in effect at every level")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("A value set for a level holds for that level and the lower ones until a level of their own sets another. On a slider it is a ceiling that never raises the user's value.")
    }

    SceneQuality {
        page: levels.page
    }

    SceneRepeater {
        id: listed

        rows: levels.info?.levels ?? []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            font.bold: modelData.quality === SceneLook.quality
            elide: Text.ElideRight
            text: (modelData.own ? App.tr("%1: %2", modelData.quality.toUpperCase(), levels.shown(modelData.value)) : App.tr("%1: %2 (inherited)", modelData.quality.toUpperCase(), levels.shown(modelData.value)))
        }
    }
}
