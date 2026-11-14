import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var grants: page.inputs.grants ?? {}
    readonly property var pairs: Object.keys(grants).reduce((all, scene) => all.concat((grants[scene] ?? []).map(input => [scene, input])), [])

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            text: App.tr("What scenes may use")
            value: MachineWords.scenes(Object.keys(part.grants).length)
            explain: App.tr("A wallpaper follows the sound that plays or the pointer, or takes clicks, only after you allow it for that wallpaper, and only while it is shown; clicks also have to be turned on for each screen in its wallpaper settings. The sound is turned into levels on this computer and never recorded, kept or sent; pointer positions and clicks are never kept.")
            status: MachineWords.following(part.page.inputs)
        }

        Vacio {
            visible: part.pairs.length === 0
            small: true
            title: App.tr("No wallpaper has been allowed yet.")
            text: App.tr("A scene asks the first time it wants the pointer, the sound or the music that plays.")
        }

        Repeater {
            model: part.pairs

            delegate: Fila {
                required property var modelData

                objectName: "dynamic"
                text: modelData[0]
                note: Labels.value(modelData[1])
                kind: "pliega"
                value: App.tr("Stop using %1", Labels.value(modelData[1]))
                explain: App.tr("The scene stops using it now and asks again the next time it wants it.")
                onOpened: App.act("Revoke", [modelData[0], modelData[1]], "Inputs1")
            }
        }
    }
}
