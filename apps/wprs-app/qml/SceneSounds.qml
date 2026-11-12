import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sounds

    required property var page
    required property var node
    required property string pointer
    readonly property bool root: pointer === "/scene/canvas"
    readonly property bool shown: root || node?.type === "sound"
    readonly property var found: shown ? ask(page.view) : ({})
    readonly property var layers: (found.layers ?? []).filter(layer => root || layer.id === node?.id)
    readonly property var spare: root ? (found.assets ?? []).filter(asset => !(found.layers ?? []).some(layer => layer.sounds.includes(asset.name))) : []
    property var waves: ({})
    property var heard: null

    function ask(view) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "sounds": {}
                })) || "{}");
    }

    function key(asset) {
        const bytes = (found.assets ?? []).find(a => a.name === asset)?.bytes ?? 0;
        return asset + ":" + bytes;
    }

    function wave(asset) {
        return waves[key(asset)] ?? ({});
    }

    function fetch() {
        const next = Object.assign({}, waves);
        let asked = false;
        for (const asset of found.assets ?? []) {
            const at = key(asset.name);
            if (next[at] === undefined) {
                next[at] = {};
                asked = true;
                page.editor.wave(asset.name, 256);
            }
        }
        if (asked)
            waves = next;
    }

    function same(a, b) {
        return JSON.stringify(a) === JSON.stringify(b);
    }

    function listen(what) {
        heard = same(heard, what) ? null : what;
        SceneLook.hear(page, heard);
    }

    function trigger(layer) {
        return layer.play_on ? App.tr("Starts with %1", layer.play_on) : App.tr("Starts with the scene");
    }

    visible: shown && (layers.length > 0 || spare.length > 0)
    onFoundChanged: fetch()
    Component.onCompleted: fetch()
    Component.onDestruction: {
        if (heard !== null && page)
            SceneLook.hear(page, null);
    }

    Connections {
        function onWaved(asset, drawn) {
            const next = Object.assign({}, sounds.waves);
            next[sounds.key(asset)] = JSON.parse(drawn);
            sounds.waves = next;
        }

        target: sounds.page.editor
    }

    Kirigami.Heading {
        level: 4
        text: sounds.root ? App.tr("Sounds of the scene") : App.tr("Sound")
    }

    Repeater {
        model: sounds.layers

        delegate: ColumnLayout {
            id: row

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
                    text: sounds.trigger(row.modelData)
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.ToolButton {
                    checkable: true
                    checked: sounds.same(sounds.heard, row.modelData.id)
                    icon.name: checked ? "media-playback-stop" : "media-playback-start"
                    text: checked ? App.tr("Stop listening") : App.tr("Listen")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: sounds.listen(row.modelData.id)
                }

                QQC2.ToolButton {
                    visible: sounds.root
                    icon.name: "go-next"
                    text: App.tr("Edit the sound layer")
                    display: QQC2.AbstractButton.IconOnly
                    onClicked: sounds.page.editor.select(row.modelData.pointer)
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
                        text: clip.modelData + (sounds.wave(clip.modelData).seconds ? " · " + App.tr("%1 s", Number(sounds.wave(clip.modelData).seconds).toFixed(1)) : "")
                        color: Kirigami.Theme.disabledTextColor
                    }

                    SceneWave {
                        Layout.fillWidth: true
                        wave: sounds.wave(clip.modelData)
                        fadeIn: clip.index === 0 ? row.modelData.fade_in : 0
                        fadeOut: clip.index === row.modelData.sounds.length - 1 ? row.modelData.fade_out : 0
                        fades: clip.index === 0 || clip.index === row.modelData.sounds.length - 1
                        onFaded: (key, seconds) => sounds.page.run("set", {
                                "pointer": row.modelData.pointer + "/" + key,
                                "value": seconds
                            })
                    }
                }
            }
        }
    }

    Kirigami.Heading {
        visible: sounds.spare.length > 0
        level: 5
        text: App.tr("Sounds no layer plays")
    }

    Repeater {
        model: sounds.spare

        delegate: ColumnLayout {
            id: loose

            required property var modelData

            Layout.fillWidth: true

            RowLayout {
                Layout.fillWidth: true

                QQC2.Label {
                    Layout.fillWidth: true
                    text: loose.modelData.name
                    elide: Text.ElideRight
                }

                QQC2.ToolButton {
                    checkable: true
                    checked: sounds.same(sounds.heard, {
                        "asset": loose.modelData.name
                    })
                    icon.name: checked ? "media-playback-stop" : "media-playback-start"
                    text: checked ? App.tr("Stop listening") : App.tr("Listen")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: sounds.listen({
                        "asset": loose.modelData.name
                    })
                }
            }

            SceneWave {
                Layout.fillWidth: true
                wave: sounds.wave(loose.modelData.name)
                fades: false
            }
        }
    }
}
