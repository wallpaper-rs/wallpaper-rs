import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}
    readonly property var hdrWords: ["auto", "on", "off"]
    readonly property string hdr: String(options["playback.hdr"] ?? "auto")

    function move(item, ahead) {
        const next = item.nextItemInFocusChain(ahead);
        if (next)
            next.forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: filas.height

    GpuChoices {
        id: gpu
    }

    Filas {
        id: filas

        Seccion {
            text: App.tr("How it is drawn")
        }

        Fila {
            text: App.tr("Graphics card")
            note: gpu.now
            kind: "elige"
            options: gpu.choices.map(choice => choice.text)
            current: gpu.current
            explain: App.tr("Automatic chooses the one that saves the most power. A dedicated card draws heavy scenes at a higher quality and uses more power. Here: any integrated card, any dedicated card or one in particular; the processor, in “For tests”.")
            status: gpu.said
            statusTone: App.state.gpu_warning ? "coral" : "ambar"
            onChosen: index => App.act("SetOption", ["render.gpu", gpu.choices[index].value])
        }

        Fila {
            text: App.tr("Restart the daemon now")
            note: App.tr("A change waits for the next start")
            visible: App.state.gpu_restart === true || (App.state.layer_shell ?? {}).restart === true
            kind: "pliega"
            onOpened: App.act("Restart", [], "Daemon1")
        }

        Fila {
            text: App.tr("HDR, extra bright colors")
            note: off ? App.tr("No connected screen supports it") : ""
            off: !MachineWords.hdrSeen(App.state.outputs) && part.hdr !== "off"
            kind: off ? "texto" : "elige"
            value: off ? App.tr("not detected") : ""
            options: [App.tr("Automatic"), App.tr("Always"), App.tr("Never")]
            current: Math.max(0, part.hdrWords.indexOf(part.hdr))
            explain: App.tr("Screens and desktops that support HDR show bright scenes brighter than white, tone mapped to how bright each screen gets. It uses more power, so the automatic choice keeps it to mains power and the highest scene quality.")
            onChosen: index => App.act("SetOption", ["playback.hdr", part.hdrWords[index]])
        }

        Fila {
            id: depth

            text: App.tr("Depth quality")
            note: part.options["models.allow_download"] === false ? App.tr("Downloads are turned off") : App.tr("What it needs is downloaded by itself, when it is needed")
            kind: "pliega"
            value: MachineWords.depth(part.options)
            onOpened: part.page.open("depth", depth)
        }

        Seccion {
            text: App.tr("Maintenance")
        }

        Item {
            implicitWidth: missing.implicitWidth
            implicitHeight: missing.implicitHeight

            Fila {
                id: missing

                anchors.fill: parent
                text: App.tr("If something does not show")
                explain: App.tr("Reload the settings reads the configuration file again. Check the whole library looks through every folder again.")
            }

            Row {
                anchors.right: parent.right
                anchors.rightMargin: Look.filaFin
                anchors.verticalCenter: parent.verticalCenter
                spacing: Look.e3

                QQC2.Button {
                    id: reload

                    text: App.tr("Reload the settings")
                    Keys.onUpPressed: event => part.move(reload, false)
                    Keys.onDownPressed: event => part.move(reload, true)
                    onClicked: App.act("Reload", [])
                }

                QQC2.Button {
                    id: rescan

                    text: App.tr("Check the whole library")
                    Keys.onUpPressed: event => part.move(rescan, false)
                    Keys.onDownPressed: event => part.move(rescan, true)
                    onClicked: part.page.sources.forEach(source => App.act("Rescan", [source.uid], "Library1"))
                }
            }
        }

        Fila {
            id: version

            objectName: "dynamic"
            text: App.tr("Version %1", App.state.version ?? "")
            note: App.state.config_error || App.state.library_error ? App.tr("The configuration has a problem") : ""
            noteTone: "coral"
            kind: "pliega"
            value: App.tr("Details")
            onOpened: part.page.open("version", version)
        }

        Fila {
            id: testing

            text: App.tr("For tests")
            note: App.tr("Name of the surface, drawing with the processor")
            kind: "pliega"
            onOpened: part.page.open("testing", testing)
        }
    }
}
