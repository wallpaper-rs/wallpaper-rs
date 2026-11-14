import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: own

    required property var daemon
    required property var row
    readonly property string name: row.name
    readonly property var info: row.info
    readonly property bool blank: info.blank === true
    property string open: ""
    readonly property var fits: [["fill", i18n("Fill")], ["fit", i18n("Fit")], ["stretch", i18n("Stretch")], ["center", i18n("Center")], ["tile", i18n("Tile")]]
    readonly property var looks: [["none", i18n("None")], ["warm", i18n("Warm")], ["cool", i18n("Cool")], ["golden", i18n("Golden")], ["vivid", i18n("Vivid")], ["punch", i18n("Punch")], ["muted", i18n("Subdued")], ["matte", i18n("Matte")], ["fade", i18n("Faded")], ["mono", i18n("Black and white")], ["noir", i18n("Noir")], ["sepia", i18n("Sepia")], ["dusk", i18n("Dusk")], ["mint", i18n("Mint")], ["rose", i18n("Rose")], ["night", i18n("Night")]]

    function label(list, value) {
        return (list.find(entry => entry[0] === value) ?? ["", ""])[1];
    }

    spacing: 0

    Toggle {
        Layout.fillWidth: true
        enabled: own.row.item !== ""
        text: i18n("Favorite")
        checked: own.row.details.favorite === true
        onToggled: {
            own.daemon.act("SetFavorite", [[own.row.item], checked], "Library1");
            checked = Qt.binding(() => own.row.details.favorite === true);
        }
    }

    Option {
        Layout.fillWidth: true
        text: i18n("Fit")
        hint: own.label(own.fits, own.info.fit ?? "fill")
        more: true
        open: own.open === "fit"
        onClicked: own.open = own.open === "fit" ? "" : "fit"
    }

    Choices {
        Layout.fillWidth: true
        visible: own.open === "fit"
        model: own.fits
        current: own.info.fit ?? "fill"
        onPicked: value => own.daemon.act("SetWallpaper", ["", own.name, value])
    }

    Option {
        Layout.fillWidth: true
        text: i18n("Filter")
        hint: own.label(own.looks, (own.info.filter ?? {}).look ?? "none")
        more: true
        open: own.open === "filter"
        onClicked: own.open = own.open === "filter" ? "" : "filter"
    }

    Choices {
        Layout.fillWidth: true
        visible: own.open === "filter"
        model: own.looks
        current: (own.info.filter ?? {}).look ?? "none"
        onPicked: value => own.daemon.act("SetFilter", [value === "none" ? "" : JSON.stringify({
                "look": value
            }), own.name])
    }

    Toggle {
        Layout.fillWidth: true
        text: i18n("Let clicks and taps on this screen reach its wallpaper")
        checked: (own.daemon.inputs.clicks ?? []).includes(own.name)
        onToggled: {
            own.daemon.act("SetClicks", [own.name, checked], "Inputs1");
            checked = Qt.binding(() => (own.daemon.inputs.clicks ?? []).includes(own.name));
        }
    }

    Option {
        Layout.fillWidth: true
        visible: !own.blank
        glyph: "broom"
        text: i18n("Remove the wallpaper of this screen")
        onClicked: own.daemon.act("ClearWallpaper", [own.name])
    }

    Option {
        Layout.fillWidth: true
        visible: own.blank
        glyph: "monitor"
        text: i18n("Go back to Plasma's wallpaper")
        onClicked: own.daemon.act("HandBackToPlasma", [])
    }

    Option {
        Layout.fillWidth: true
        visible: own.blank && own.info.previous === true
        glyph: "arrow-counter-clockwise"
        text: i18n("Bring back the previous one")
        onClicked: own.daemon.act("RestoreWallpaper", [own.name])
    }
}
