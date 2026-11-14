import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: dialog

    property string item
    property string shade
    readonly property var fits: [""].concat(App.schema.fits ?? ["fill", "fit", "stretch", "center", "tile"])

    title: item ? App.tr("New preset for this item") : App.tr("New preset")
    quiet: App.tr("Cancel")
    principal: App.tr("OK")
    onAccepted: {
        const values = {};
        if (fit.currentValue)
            values.fit = fit.currentValue;
        if (shade)
            values.background = shade;
        App.act("SavePreset", [item, name.text, values], "Profiles1");
    }

    GridLayout {
        width: dialog.width - 2 * Look.e4
        columns: 2

        QQC2.Label {
            text: App.tr("Name:")
        }

        QQC2.TextField {
            id: name

            Layout.fillWidth: true
            Accessible.name: App.tr("Name:")
        }

        QQC2.Label {
            text: Labels.field("fit") + ":"
        }

        QQC2.ComboBox {
            id: fit

            Layout.fillWidth: true
            Accessible.name: Labels.field("fit")
            model: dialog.fits
            displayText: currentValue ? Labels.value(currentValue) : App.tr("Keep")
            delegate: QQC2.ItemDelegate {
                required property string modelData
                required property int index

                text: modelData ? Labels.value(modelData) : App.tr("Keep")
                highlighted: fit.highlightedIndex === index
            }
        }

        QQC2.Label {
            text: Labels.field("background") + ":"
        }

        QQC2.Button {
            Accessible.name: Labels.field("background")
            text: dialog.shade || App.tr("Keep")
            icon.name: "color-picker"
            onClicked: colors.open()
        }
    }

    ColorDialog {
        id: colors

        onAccepted: dialog.shade = selectedColor.toString()
    }
}
