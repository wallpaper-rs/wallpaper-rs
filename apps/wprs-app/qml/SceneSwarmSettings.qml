import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: settings

    required property var tools
    readonly property var spaces: ["local", "world"]
    readonly property var kits: ["snow", "sparks", "rain", "dust"]
    readonly property real capacity: read("/capacity", 1)
    readonly property real seed: read("/seed", 0)
    readonly property real prewarm: read("/prewarm", 0)
    readonly property string space: read("/space", "local")

    function read(rest, fallback) {
        tools.info;
        const value = JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, rest)) || "null");
        return value ?? fallback;
    }

    function put(values, gesture) {
        tools.edit("settings", values, gesture);
    }

    function seeded(text) {
        const number = Math.floor(Number(text));
        if (text !== "" && isFinite(number) && number >= 0 && number <= 9007199254740991 && number !== seed)
            put({
                    "seed": number
                });
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Settings of this system")
    }

    GridLayout {
        Layout.fillWidth: true
        columns: 2

        QQC2.Label {
            text: App.tr("Start with")
        }

        QQC2.ComboBox {
            Layout.fillWidth: true
            model: [App.tr("Snow"), App.tr("Sparks"), App.tr("Rain"), App.tr("Dust")]
            currentIndex: -1
            displayText: App.tr("Choose a ready-made start…")
            onActivated: index => settings.tools.edit("kit", {
                    "kit": settings.kits[index]
                })
        }

        QQC2.Label {
            text: App.tr("Most particles alive")
        }

        QQC2.SpinBox {
            Layout.fillWidth: true
            from: 1
            to: 200000
            stepSize: 100
            editable: true
            value: settings.capacity
            onValueModified: settings.put({
                    "capacity": value
                })
        }

        QQC2.Label {
            text: App.tr("Seed")
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                Layout.fillWidth: true
                text: String(settings.seed)
                inputMethodHints: Qt.ImhDigitsOnly
                validator: RegularExpressionValidator {
                    regularExpression: /[0-9]{1,16}/
                }
                onEditingFinished: settings.seeded(text)
            }

            QQC2.ToolButton {
                icon.name: "roll"
                text: App.tr("Pick a random seed")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: text
                onClicked: settings.put({
                        "seed": Math.floor(Math.random() * 4294967296)
                    })
            }

            QQC2.ToolButton {
                visible: (settings.tools.info?.seed ?? 0) !== settings.seed
                icon.name: "object-locked"
                text: App.tr("Use the seed of the preview")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: text
                onClicked: settings.put({
                        "seed": settings.tools.info?.seed ?? 0
                    })
            }
        }

        QQC2.Label {
            text: App.tr("Seconds simulated before it shows %1", Math.round(settings.prewarm * 10) / 10)
        }

        SceneSlider {
            Layout.fillWidth: true
            from: 0
            to: Math.max(10, settings.prewarm)
            value: settings.prewarm
            onMoved: settings.put({
                    "prewarm": Math.round(value * 10) / 10
                }, true)
        }

        QQC2.Label {
            text: App.tr("Particles move with")
        }

        QQC2.ComboBox {
            Layout.fillWidth: true
            model: [App.tr("The layer"), App.tr("Where they were born")]
            currentIndex: Math.max(0, settings.spaces.indexOf(settings.space))
            onActivated: index => settings.put({
                    "space": settings.spaces[index]
                })
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: settings.space === "world" ? App.tr("The layer and the control points only place particles when they are born") : App.tr("Particles move with the layer that draws them")
    }
}
