import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: lights

    property var page: null
    property var status: ({})
    property string problem: ""
    property var options: App.state.options ?? {}
    readonly property bool on: options["rgb.enabled"] === true
    readonly property string server: String(options["rgb.server"] ?? "127.0.0.1:6742")
    readonly property string covered: String(options["rgb.when_covered"] ?? "pause")
    readonly property bool pauses: options["pause.when_covered"] !== false
    readonly property var devices: status.devices ?? []
    readonly property alias coveredRow: coveredRow

    function load() {
        App.query("GetStatus", [], "Rgb1", reply => lights && (lights.status = reply ?? {}), error => lights && (lights.problem = error.message));
    }

    function ask(member, args) {
        problem = "";
        App.act(member, args, "Rgb1", () => lights && lights.load(), error => lights && lights.failed(error));
    }

    function failed(error) {
        problem = error.message;
        load();
    }

    function set(key, value) {
        App.act("SetOption", [key, value], "Daemon1", () => lights && lights.load(), error => lights && (lights.problem = error.message));
    }

    implicitHeight: filas.height + board.height + Look.e4
    Component.onCompleted: load()

    Timer {
        interval: 2000
        repeat: true
        running: lights.visible && lights.on
        onTriggered: lights.load()
    }

    Filas {
        id: filas

        Fila {
            text: App.tr("Send colors to RGB lighting")
            kind: "llave"
            checked: lights.on
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Wallpapers that send colors to keyboards, mice and other lights reach them through an OpenRGB server, and only after you allow it for each package. Turn this on to let them; the address is this computer unless you change it.")
            onToggled: lights.set("rgb.enabled", String(checked))
        }

        Fila {
            text: App.tr("OpenRGB server")
            kind: "campo"
            fieldText: lights.server
            placeholder: "127.0.0.1:6742"
            explain: lights.status.local === false ? App.tr("The colors of your wallpaper go over the network to %1. Anyone who can watch that network could see them; use an address on this computer to keep them here.", lights.server) : ""
            status: lights.problem || lights.status.error || (lights.status.connected ? App.tr("Connected; %1 sends the colors now.", lights.status.source ?? App.tr("nothing")) : App.tr("Not connected."))
            statusTone: lights.problem || lights.status.error ? "coral" : lights.status.local === false ? "ambar" : ""
            onEdited: text => {
                if (text !== lights.server)
                    lights.set("rgb.server", text);
            }
        }

        Fila {
            text: App.tr("Connect")
            kind: "pliega"
            off: !lights.on
            onOpened: lights.ask("Connect", [])
        }

        Fila {
            id: coveredRow

            text: App.tr("While a window covers the wallpaper and it pauses")
            kind: "elige"
            off: !lights.pauses
            options: [App.tr("Keep following the wallpaper"), App.tr("Pause and leave the lights as they are")]
            current: lights.covered === "follow" ? 0 : 1
            explain: !lights.pauses ? App.tr("Pausing while a window covers the screen is off, so the covered wallpaper keeps drawing and the lights keep following it whatever you choose here.") : lights.covered === "follow" ? App.tr("While the covered wallpaper pauses, the lights keep changing with the scene or page, which keeps running without being drawn, as long as the server has devices connected.") : App.tr("While the covered wallpaper pauses, nothing is sent and nothing runs for the lights until it shows again.")
            onChosen: index => lights.set("rgb.when_covered", index === 0 ? "follow" : "pause")
        }

        Repeater {
            model: Object.keys(lights.status.waiting ?? {})

            delegate: Fila {
                required property string modelData
                readonly property string why: String(lights.status.waiting[modelData])

                objectName: "dynamic"
                text: modelData
                value: App.tr("Waiting")
                explain: why === "locked" ? App.tr("%1: the lights wait while the session is locked, because the wallpaper pauses then.", modelData) : why === "battery" ? App.tr("%1: the lights wait while the computer is on battery, because the wallpaper pauses then.", modelData) : why === "colorless" ? App.tr("%1: the lights wait, the covered screen shows something that sends no colors.", modelData) : why === "ungranted" ? App.tr("%1: the lights wait, the wallpaper there does not have permission to use the lights.", modelData) : App.tr("%1: the lights wait while the wallpaper is paused.", modelData)
            }
        }

        Repeater {
            model: lights.devices

            delegate: Fila {
                required property var modelData

                objectName: "dynamic"
                text: modelData.key ?? modelData.name
                note: !modelData.own_mode ? App.tr("%1 lights", modelData.leds) : modelData.taken ? App.tr("%1 lights; its own mode %2 comes back when the lights stop", modelData.leds, modelData.own_mode) : App.tr("%1 lights, in its own mode %2", modelData.leds, modelData.own_mode)
                kind: "pliega"
                value: App.tr("Test")
                off: lights.status.testing === true
                onOpened: lights.ask("Test", [modelData.index])
            }
        }

        Fila {
            text: App.tr("Test every device")
            visible: lights.devices.length > 1
            kind: "pliega"
            off: lights.status.testing === true
            onOpened: lights.ask("Test", [-1])
        }
    }

    RgbBoard {
        id: board

        x: Look.filaLado
        y: filas.height + Look.e4
        width: filas.width - Look.filaLado * 2
        visible: lights.devices.length > 0
        height: visible ? implicitHeight : 0
        status: lights.status
        onSaved: lights.load()
        onFailed: error => lights.failed(error)
    }
}
