import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: look

    required property var page
    readonly property var bloom: page.view.canvas?.bloom ?? null
    readonly property var glide: page.view.canvas?.parallax ?? null
    readonly property var glows: [["threshold", App.tr("Threshold"), 0, 4, 0.8], ["strength", App.tr("Strength"), 0, 8, 1], ["radius", App.tr("Radius"), 0, 128, 16]]
    readonly property var moves: [["amount", App.tr("Depth"), 0, 4, 1], ["delay", App.tr("Delay"), 0, 2, 0.15], ["pointer", App.tr("Pointer"), 0, 1, 1], ["offset", App.tr("Offset"), -1, 1, 0]]

    function field(key, fallback) {
        const value = glide?.[key];
        const bound = value !== null && typeof value === "object" && !Array.isArray(value);
        return {
            "pointer": "/scene/canvas/parallax/" + key,
            "source": bound ? (["keys", "prop", "audio", "cursor", "clock", "script", "media"].find(k => k in value) ?? "prop") : "",
            "value": value,
            "default": key === "offset" ? [0, 0] : fallback,
            "widget": key === "offset" ? "vec2" : "number"
        };
    }

    function shifted(axis, value) {
        const now = Array.isArray(glide?.offset) ? glide.offset : [0, 0];
        const next = axis === 0 ? [Math.round(value * 100) / 100, now[1]] : [now[0], Math.round(value * 100) / 100];
        page.run("canvas", {
            "parallax": {
                "offset": next
            }
        }, true);
    }

    function edit(what, args) {
        page.run("canvas", {
            [what]: args
        });
    }

    function slid(what, key, value, fallback) {
        const stored = (what === "bloom" ? bloom : glide)?.[key];
        const rounded = Math.round(value * 100) / 100;
        if (rounded !== (typeof stored === "number" ? stored : fallback))
            page.run("canvas", {
                [what]: {
                    [key]: rounded
                }
            }, true);
    }

    SceneValueRow {
        text: App.tr("Bright parts glow")

        QQC2.Switch {
            Accessible.name: App.tr("Bright parts glow")
            checked: look.bloom !== null
            onToggled: look.edit("bloom", checked ? {} : {
                "off": true
            })
        }
    }

    Repeater {
        model: look.glows

        delegate: SceneValueRow {
            id: glow

            required property var modelData

            visible: look.bloom !== null
            text: glow.modelData[1]

            SceneSlider {
                Layout.fillWidth: true
                Layout.preferredWidth: Look.rielAncho
                Accessible.name: glow.text
                from: glow.modelData[2]
                to: glow.modelData[3]
                stepSize: (glow.modelData[3] - glow.modelData[2]) / 100
                value: typeof look.bloom?.[glow.modelData[0]] === "number" ? look.bloom[glow.modelData[0]] : glow.modelData[4]
                onMoved: look.slid("bloom", glow.modelData[0], value, glow.modelData[4])
            }
        }
    }

    SceneValueRow {
        text: App.tr("Layers move in depth")

        QQC2.Switch {
            Accessible.name: App.tr("Layers move in depth")
            checked: look.glide !== null
            onToggled: look.edit("parallax", checked ? {} : {
                "off": true
            })
        }
    }

    Repeater {
        model: look.moves

        delegate: SceneValueRow {
            id: move

            required property var modelData
            readonly property var held: look.glide?.[modelData[0]]

            visible: look.glide !== null
            text: modelData[1]

            Repeater {
                model: move.modelData[0] === "offset" ? [0, 1] : [-1]

                delegate: SceneSlider {
                    required property int modelData

                    Layout.fillWidth: true
                    Layout.preferredWidth: modelData >= 0 ? Look.rielAncho / 2 : Look.rielAncho
                    Accessible.name: move.text
                    enabled: typeof move.held !== "object" || Array.isArray(move.held) || move.held === undefined
                    from: move.modelData[2]
                    to: move.modelData[3]
                    stepSize: (move.modelData[3] - move.modelData[2]) / 100
                    value: modelData >= 0 ? (Array.isArray(move.held) ? move.held[modelData] : 0) : (typeof move.held === "number" ? move.held : move.modelData[4])
                    onMoved: modelData >= 0 ? look.shifted(modelData, value) : look.slid("parallax", move.modelData[0], value, move.modelData[4])
                }
            }

            SceneBind {
                visible: move.modelData[0] !== "delay"
                page: look.page
                field: look.field(move.modelData[0], move.modelData[4])
            }
        }
    }

    SceneValueRow {
        visible: look.glide !== null
        text: App.tr("Follow the tilt of a phone")

        QQC2.CheckBox {
            Accessible.name: App.tr("Follow the tilt of a phone")
            checked: look.glide?.gyro === true
            onToggled: look.edit("parallax", {
                "gyro": checked
            })
        }
    }
}
