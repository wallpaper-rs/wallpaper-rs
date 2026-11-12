import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: accent

    required property var owner
    readonly property var value: owner.field.value
    readonly property int mode: typeof value === "string" ? 1 : value && value.prop !== undefined ? 2 : 0
    readonly property var colors: owner.field.options

    function choose(next) {
        owner.page.run("manifest", {
                "display": {
                    "values": {
                        "accent": next
                    }
                }
            });
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: [App.tr("Dominant color of the preview"), App.tr("A fixed color"), App.tr("The color of a property")]
        currentIndex: accent.mode
        Accessible.name: App.tr("Accent color")
        onActivated: index => {
            if (index === 0)
                accent.owner.page.run("manifest", {
                        "display": {
                            "clear": ["accent"]
                        }
                    });
            else if (index === 1)
                accent.choose(typeof accent.value === "string" ? accent.value : "#3daee9");
            else if (accent.colors.length)
                accent.choose({
                        "prop": accent.colors[0]
                    });
        }
    }

    SceneColor {
        visible: accent.mode === 1
        alpha: false
        hex: typeof accent.value === "string" ? accent.value : "#3daee9"
        onPicked: hex => accent.choose(hex)
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        visible: accent.mode === 2
        model: accent.colors
        currentIndex: accent.colors.indexOf(accent.value?.prop)
        Accessible.name: App.tr("Color property")
        onActivated: index => accent.choose({
                "prop": accent.colors[index]
            })
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: accent.mode === 2 && accent.colors.length === 0
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("Add a color property first; the accent then follows its value.")
    }
}
