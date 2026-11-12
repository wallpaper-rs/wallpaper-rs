import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: rate

    required property var owner
    readonly property var value: owner.field.value
    readonly property var steps: [10, 15, 24, 30, 48, 60, 90, 120, 144, 240]
    readonly property int custom: steps.length + 2
    readonly property int mode: value === undefined || value === null ? 0 : value === "display" ? 1 : steps.includes(value) ? steps.indexOf(value) + 2 : custom
    property bool typing: false

    function put(next) {
        owner.page.run("manifest", {
                "playback": {
                    "values": {
                        "max_fps": next
                    }
                }
            });
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        Accessible.name: App.tr("Highest frame rate")
        model: [App.tr("Not set"), App.tr("Follow the screen")].concat(rate.steps.map(r => App.tr("%1 frames a second", r))).concat([App.tr("Another rate…")])
        currentIndex: rate.typing ? rate.custom : rate.mode
        onActivated: index => {
            rate.typing = index === rate.custom;
            if (index === 0)
                rate.owner.page.run("manifest", {
                        "playback": {
                            "clear": ["max_fps"]
                        }
                    });
            else if (index === 1)
                rate.put("display");
            else if (index < rate.custom)
                rate.put(rate.steps[index - 2]);
            else if (rate.mode !== rate.custom)
                rate.put(Number(rate.value) > 0 ? rate.value : 30);
        }
    }

    RowLayout {
        visible: rate.mode === rate.custom || rate.typing

        QQC2.SpinBox {
            from: 1
            to: 240
            editable: true
            value: Number(rate.value) > 0 ? rate.value : 30
            Accessible.name: App.tr("Frames a second")
            onValueModified: rate.put(value)
        }

        QQC2.Label {
            text: App.tr("frames a second")
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("A ceiling: what the user or the battery sets is never exceeded, and the content never goes above its own rate.")
    }
}
