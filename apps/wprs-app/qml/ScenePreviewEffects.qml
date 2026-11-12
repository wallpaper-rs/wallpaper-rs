import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: effects

    required property var page
    readonly property string stamp: SceneLook.quality + SceneLook.battery + SceneLook.calm + effects.visible
    property var info: null
    readonly property var followers: (effects.info?.effects ?? []).filter(e => e.follows).map(e => SceneWords.effect(e.effect) + " (" + e.layer + ")").concat(effects.info?.pointer?.emitters ?? [])

    function load() {
        if (!effects.visible)
            return;
        effects.info = JSON.parse(effects.page.editor.rig("", JSON.stringify({
                    "level": {
                        "quality": SceneLook.quality,
                        "battery": SceneLook.battery,
                        "calm": SceneLook.calm
                    }
                })) || "null");
    }

    onStampChanged: load()
    Component.onCompleted: load()

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: effects.info !== null
        text: App.tr("Drawn as %1 at %2 % of the size, with %3 effects left out", (effects.info?.quality ?? "").toUpperCase(), Math.round((effects.info?.render_scale ?? 1) * 100), effects.info?.left_out ?? 0)
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: SceneLook.battery
        text: App.tr("On battery the daemon lowers Q3 to Q2 and draws at most 60 frames a second")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: SceneLook.calm
        text: App.tr("With reduced motion the parallax and the camera shake stand still")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: effects.info?.bloom?.hdr ?? false
        text: effects.info?.bloom?.glare ? App.tr("The glare above white is drawn") : App.tr("The glare above white is left out")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: effects.followers.length > 0
        text: App.tr("Following the pointer, try them with Test the pointer and clicks: %1", effects.followers.join(", "))
    }

    SceneRepeater {
        id: listed

        rows: effects.info?.effects ?? []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            font.bold: !modelData.kept
            text: modelData.kept ? App.tr("%1 on %2: %3 units of work", SceneWords.effect(modelData.effect), modelData.layer, modelData.units.toFixed(2)) : App.tr("%1 on %2: left out of the scene at this level", SceneWords.effect(modelData.effect), modelData.layer)
        }
    }
}
