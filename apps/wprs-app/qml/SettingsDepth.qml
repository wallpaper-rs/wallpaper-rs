import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property var fills: ["lama", "simple", "off"]
    readonly property var layers: [4, 5, 6]

    implicitHeight: filas.height + note.implicitHeight + Look.e4

    Filas {
        id: filas

        Fila {
            text: App.tr("Allow downloading the depth model")
            note: App.tr("Downloaded once, when a picture first needs it")
            kind: "llave"
            checked: part.options["models.allow_download"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: App.act("SetOption", ["models.allow_download", String(checked)])
        }

        Fila {
            text: App.tr("Fill behind near objects")
            kind: "elige"
            options: part.fills.map(fill => MachineWords.fills[fill])
            current: Math.max(0, part.fills.indexOf(String(part.options["parallax.fill"] ?? "lama")))
            explain: App.tr("Smart fill paints what a near object hid when it moves; simple fill stretches the edges; no fill leaves the gap.")
            onChosen: index => App.act("SetOption", ["parallax.fill", part.fills[index]])
        }

        Fila {
            text: App.tr("Depth layers")
            kind: "elige"
            options: part.layers.map(count => App.tr("%1 layers", count))
            current: Math.max(0, part.layers.indexOf(Number(part.options["parallax.layers"] ?? 5)))
            explain: App.tr("More layers separate near and far better and take longer to compute.")
            onChosen: index => App.act("SetOption", ["parallax.layers", String(part.layers[index])])
        }
    }

    FillNote {
        id: note

        x: Look.filaLado
        y: filas.height + Look.e4
        width: filas.width - Look.filaLado * 2
    }
}
