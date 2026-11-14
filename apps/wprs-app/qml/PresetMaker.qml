import QtQuick
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

PanelChico {
    id: maker

    property string name
    property string shade
    readonly property var fits: [""].concat(App.schema.fits ?? ["fill", "fit", "stretch", "center", "tile"])
    property int fit: 0

    function start(from) {
        opener = from;
        name = "";
        shade = "";
        fit = 0;
        open();
    }

    function save() {
        const values = {};
        if (fits[fit])
            values.fit = fits[fit];
        if (shade)
            values.background = shade;
        App.act("SavePreset", ["", name.trim(), values], "Profiles1");
    }

    title: App.tr("New preset")
    text: App.tr("It keeps how a scene is placed and the fill color shown where the picture does not reach. It works for every scene.")
    quiet: App.tr("Cancel")
    principal: App.tr("Save")
    first: field
    onAccepted: save()

    Fila {
        id: field

        width: parent.width - 2 * Look.e4
        kind: "campo"
        text: App.tr("Name")
        placeholder: App.tr("For example, Cinema")
        fieldText: maker.name
        onEdited: text => maker.name = text
    }

    Fila {
        width: parent.width - 2 * Look.e4
        kind: "elige"
        text: App.tr("Placement")
        options: maker.fits.map(fit => fit ? Labels.value(fit) : App.tr("Keep"))
        current: maker.fit
        onChosen: index => maker.fit = index
    }

    Fila {
        width: parent.width - 2 * Look.e4
        kind: "pliega"
        text: App.tr("Fill color")
        value: maker.shade || App.tr("Keep")
        onOpened: colors.open()
    }

    ColorDialog {
        id: colors

        onAccepted: maker.shade = selectedColor.toString()
    }
}
