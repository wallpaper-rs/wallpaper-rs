import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: panel

    required property var page
    readonly property var tool: page.settings ? page.settings.passage : null
    readonly property var current: tool ? tool.current : null
    readonly property var limits: tool && tool.info.limits ? tool.info.limits : ({
            "min_ms": 100,
            "max_ms": 10000
        })
    readonly property int level: Math.max(0, SceneLook.levels.indexOf(SceneLook.effective))
    readonly property bool plays: current !== null && (current.plays ?? [true, true, true, true])[level] === true
    readonly property string wayName: tool && tool.way === "out" ? App.tr("When it leaves") : App.tr("When it comes in")
    readonly property string scene: page.title ?? ""

    visible: tool !== null && tool.open
    color: Look.nocheAlta

    QQC2.ScrollView {
        anchors.fill: parent
        anchors.leftMargin: Look.linea + Look.e4
        anchors.rightMargin: Look.e3
        anchors.topMargin: Look.e3
        contentWidth: availableWidth

        ColumnLayout {
            width: parent.width
            spacing: Look.e2

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("Options › Transitions › %1", panel.wayName)
                color: Look.bruma
                font.pixelSize: Look.tPie
                elide: Text.ElideRight
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: panel.current ? panel.tool.named(panel.current.name, App.tr("Own effect")) : ""
                font.pixelSize: Look.tValor
                font.weight: Font.DemiBold
                wrapMode: Text.WordWrap
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: panel.tool && panel.tool.way === "out" ? App.tr("Used when someone leaves «%1»", panel.scene) : App.tr("Used when someone moves to «%1»", panel.scene)
                color: Look.bruma
                wrapMode: Text.WordWrap
            }

            QQC2.Label {
                text: App.tr("Effect")
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                model: panel.tool && panel.tool.info.effects ? panel.tool.info.effects : []
                currentIndex: panel.current ? model.indexOf(panel.current.descriptor) : -1
                Accessible.name: App.tr("Effect")
                onActivated: index => panel.tool.act({
                        "effect": model[index]
                    })
            }

            QQC2.Label {
                text: App.tr("Duration")
            }

            RowLayout {
                Layout.fillWidth: true

                QQC2.SpinBox {
                    id: duration

                    from: panel.limits.min_ms / 100
                    to: panel.limits.max_ms / 100
                    value: panel.current ? Math.round(panel.current.duration_ms / 100) : 12
                    editable: true
                    Accessible.name: App.tr("Duration")
                    textFromValue: (v, locale) => Number(v / 10).toLocaleString(locale, "f", 1)
                    valueFromText: (text, locale) => Math.round(Number.fromLocaleString(locale, text) * 10)
                    onValueModified: panel.tool.act({
                        "duration_ms": value * 100
                    })
                }

                QQC2.Label {
                    text: App.tr("s")
                }
            }

            Nota {
                Layout.fillWidth: true
                wraps: true
                visible: panel.tool !== null && panel.tool.said !== ""
                tone: "coral"
                text: panel.tool ? panel.tool.said : ""
            }

            Repeater {
                model: panel.current ? (panel.current.fields ?? []).filter(f => f.type === "float") : []

                delegate: ColumnLayout {
                    id: field

                    required property var modelData
                    readonly property var set: panel.current.params ?? {}

                    Layout.fillWidth: true
                    spacing: 0

                    QQC2.Label {
                        text: panel.tool.named(field.modelData.label, field.modelData.id)
                    }

                    QQC2.Slider {
                        Layout.fillWidth: true
                        from: field.modelData.min ?? 0
                        to: field.modelData.max ?? 1
                        value: field.set[field.modelData.id] ?? field.modelData["default"] ?? 0
                        Accessible.name: panel.tool.named(field.modelData.label, field.modelData.id)
                        onPressedChanged: {
                            if (pressed)
                                return;
                            const params = {};
                            params[field.modelData.id] = Math.round(value * 1000) / 1000;
                            panel.tool.act({
                                "params": params
                            });
                        }
                    }
                }
            }

            Nota {
                Layout.fillWidth: true
                wraps: true
                visible: panel.current !== null && (panel.current.fields ?? []).some(f => f.type === "vec2")
                text: App.tr("Move the points on the canvas to place it.")
            }

            Nota {
                Layout.fillWidth: true
                wraps: true
                visible: panel.current !== null
                tone: panel.plays ? "" : "ambar"
                text: App.tr("It looks as in «See as»: %1. Change the computer above to try the others.", SceneLook.names[panel.level]) + (panel.plays ? "" : " " + App.tr("On this computer whoever uses it sees a simple fade."))
            }

            Nota {
                Layout.fillWidth: true
                wraps: true
                text: App.tr("With «Less motion» whoever uses it sees a simple fade.")
            }

            Nota {
                Layout.fillWidth: true
                wraps: true
                visible: panel.current !== null && (panel.current.problem ?? null) !== null
                tone: "coral"
                text: panel.current && panel.current.problem ? panel.current.problem : ""
            }

            QQC2.Button {
                text: App.tr("Remove the transition")
                icon.name: "trash"
                onClicked: panel.tool.remove()
            }
        }
    }
}
