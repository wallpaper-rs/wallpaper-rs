import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: top

    required property var preview
    readonly property bool several: preview.screens.length > 1

    spacing: Look.e3

    Text {
        anchors.right: parent.right
        visible: !top.several && top.preview.current !== null
        text: App.tr("Now on your desktop: %1").arg(top.preview.current ? top.preview.current.title : "")
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    Row {
        anchors.right: parent.right
        visible: top.several
        spacing: Look.e4

        Repeater {
            model: top.several ? top.preview.screens : []

            delegate: Column {
                id: screen

                required property string modelData
                readonly property var shown: App.state.outputs ? App.state.outputs[modelData] ?? ({}) : ({})
                readonly property bool chosen: !!top.preview && (top.preview.output === "" || top.preview.output === modelData)

                spacing: Look.e2
                opacity: chosen ? 1 : PreviewLook.tiraApagada

                Item {
                    width: PreviewLook.cuadro * (screen.shown.width ?? 16) / Math.max(1, screen.shown.height ?? 9)
                    height: PreviewLook.cuadro

                    PreviewPicture {
                        anchors.fill: parent
                        corner: Look.radioChico
                        item: screen.shown.item ? {
                            "uid": screen.shown.item,
                            "kind": screen.shown.kind
                        } : null
                    }

                    Anillo {
                        visible: screen.chosen
                        corner: Look.radioChico
                    }
                }

                Text {
                    text: screen.modelData
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tPie
                    color: screen.chosen ? Look.escarcha : Look.bruma
                }
            }
        }
    }

    Nota {
        anchors.right: parent.right
        visible: top.several
        icon: "monitor"
        text: top.preview.output === "" ? App.tr("Changes: every screen") : App.tr("Changes: %1").arg(top.preview.output)
    }
}
