import QtQuick
import io.github.wallpaper_rs.app

Row {
    id: strip

    property string current
    readonly property var screens: App.state.outputs ?? {}

    signal picked(string name)

    spacing: Look.e4
    Accessible.role: Accessible.RadioButton
    Accessible.name: App.tr("Which screen")

    Repeater {
        model: App.outputs

        delegate: Column {
            id: screen

            required property string modelData
            readonly property var info: strip.screens[modelData] ?? {}
            readonly property string item: info.item ?? ""
            readonly property bool chosen: modelData === strip.current
            readonly property real wide: (info.width ?? 16) / Math.max(1, info.height ?? 10)
            property string thumb

            function load() {
                const uid = item;
                thumb = "";
                if (uid !== "")
                    App.query("Thumbnail", [uid, 256], "Library1", reply => screen && screen.item === uid && (screen.thumb = reply[0] ?? ""));
            }

            anchors.bottom: parent ? parent.bottom : undefined
            spacing: Look.e2
            activeFocusOnTab: true
            Accessible.role: Accessible.RadioButton
            Accessible.name: modelData
            Accessible.checked: chosen
            onItemChanged: load()
            Component.onCompleted: load()
            Keys.onReturnPressed: strip.picked(modelData)
            Keys.onSpacePressed: strip.picked(modelData)

            Rectangle {
                id: frame

                anchors.horizontalCenter: parent.horizontalCenter
                width: screen.wide >= 16 / 9 ? Look.pantallaAncha : Look.pantalla
                height: Math.round(width / Math.max(1, screen.wide))
                radius: Look.radioChico
                color: Look.nocheAlta
                border.width: Look.anillo
                border.color: Look.nocheLinea

                Foto {
                    anchors.fill: parent
                    anchors.margins: Look.anillo
                    source: screen.thumb !== "" ? App.url(screen.thumb) : ""
                }

                Rectangle {
                    anchors.fill: parent
                    anchors.margins: Look.anillo
                    color: Look.noche
                    opacity: screen.chosen ? Look.veloTapaElegida : Look.veloTapa
                }

                Anillo {
                    visible: screen.activeFocus
                    corner: Look.radioChico
                }

                TapHandler {
                    onTapped: strip.picked(screen.modelData)
                }
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter
                text: screen.modelData
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                font.weight: screen.chosen ? Font.DemiBold : Font.Normal
                color: screen.chosen ? Look.escarcha : Look.bruma
            }

            Rectangle {
                anchors.horizontalCenter: parent.horizontalCenter
                visible: screen.chosen
                width: Look.e4
                height: Look.marcaPantalla
                radius: Look.radioChico / 2
                color: Look.luna
            }
        }
    }
}
