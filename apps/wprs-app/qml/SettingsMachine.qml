import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: tab

    required property var page
    property var lightsStatus: ({})
    readonly property var options: App.state.options ?? {}
    readonly property var inputs: page.inputs
    readonly property int granted: Object.keys(inputs.grants ?? {}).length
    readonly property bool lightsOn: options["rgb.enabled"] === true
    readonly property var rows: [
        {
            "text": grants.text,
            "value": grants.value
        },
        {
            "text": sound.text,
            "value": sound.value
        },
        {
            "text": media.text,
            "value": media.value
        },
        {
            "text": lights.text,
            "value": lights.value,
            "off": lights.off
        },
        {
            "text": advanced.text,
            "value": advanced.value
        }
    ].concat(SettingsMachineRows.places())

    function loadLights() {
        App.query("GetStatus", [], "Rgb1", reply => tab && (tab.lightsStatus = reply ?? {}));
    }

    implicitHeight: filas.height
    onVisibleChanged: if (visible)
        loadLights()
    Component.onCompleted: loadLights()

    Filas {
        id: filas

        Fila {
            id: grants

            text: App.tr("Scene permissions")
            kind: "pliega"
            value: MachineWords.scenes(tab.granted)
            explain: App.tr("What each scene can use of this computer: the pointer, the sound, the music that plays. You take it back from here whenever you want.")
            status: MachineWords.following(tab.inputs)
            onOpened: tab.page.open("grants", grants)
        }

        Fila {
            id: sound

            text: App.tr("Sound of the scenes")
            kind: "pliega"
            value: MachineWords.sound(tab.options)
            onOpened: tab.page.open("sound", sound)
        }

        Fila {
            id: media

            text: App.tr("What you are listening to")
            note: App.tr("Scenes can show the song that plays")
            kind: "pliega"
            value: tab.options["media.remote_covers"] === true ? App.tr("With covers from the internet") : App.tr("Without covers from the internet")
            onOpened: tab.page.open("media", media)
        }

        Fila {
            id: lights

            text: App.tr("RGB lights")
            note: off ? App.tr("They are handled by OpenRGB, which is not running on this computer") : App.tr("Keyboards, mice and other lights follow the wallpaper")
            kind: off ? "texto" : "pliega"
            off: !MachineWords.lightsFound(tab.lightsOn, tab.lightsStatus)
            value: MachineWords.lights(tab.lightsOn, tab.lightsStatus)
            explain: off ? App.tr("Start OpenRGB and its server, or search for “OpenRGB” to change its address or turn the lights off.") : ""
            onOpened: tab.page.open("lights", lights)
        }

        Fila {
            id: advanced

            text: App.tr("Advanced")
            kind: "pliega"
            value: App.tr("Graphics card, HDR, version")
            onOpened: tab.page.open("advanced", advanced)
        }
    }
}
