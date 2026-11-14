import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: filtro

    property var words: []
    property int current: 0
    property bool sections: false
    property string label
    property int keyed: current

    signal picked(int index)

    implicitWidth: row.implicitWidth
    implicitHeight: row.implicitHeight
    activeFocusOnTab: true
    Accessible.role: Accessible.PageTabList
    Accessible.name: label

    Row {
        id: row

        spacing: Look.e1

        Repeater {
            id: repeater

            model: filtro.words

            Item {
                id: word

                required property var modelData
                required property int index
                readonly property bool first: index === 0
                readonly property bool lit: index === filtro.current
                readonly property int side: filtro.sections ? Look.seccionesLado : Look.filtroLado
                readonly property bool keyed: filtro.activeFocus && index === filtro.keyed

                implicitWidth: line.implicitWidth + (first ? 0 : side) + side
                implicitHeight: filtro.sections ? Look.menuAlto : Look.altoControl
                Accessible.role: Accessible.PageTab
                Accessible.name: modelData.count === undefined ? modelData.text : modelData.text + " " + modelData.count
                Accessible.selected: lit
                Accessible.onPressAction: filtro.pick(index)

                Row {
                    id: line

                    x: word.first ? 0 : word.side
                    anchors.verticalCenter: parent.verticalCenter
                    spacing: Look.filtroHueco

                    Text {
                        text: word.modelData.text
                        font.family: Look.letraTitulo
                        font.pixelSize: filtro.sections ? Look.tFila : Look.tFiltro
                        font.weight: word.lit ? Font.Bold : Font.Medium
                        color: word.lit || hover.hovered ? Look.escarcha : Look.bruma
                    }

                    Text {
                        visible: word.modelData.count !== undefined
                        anchors.baseline: parent.children[0].baseline
                        text: word.modelData.count ?? ""
                        font.family: Look.letraTexto
                        font.pixelSize: Look.tCuenta
                        font.weight: Font.Medium
                        color: word.modelData.tone === "coral" ? Look.coral : Look.bruma
                    }
                }

                Rectangle {
                    visible: word.lit
                    x: line.x
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: Look.marcaAbajo
                    width: Look.marcaAncho
                    height: Look.marcaAlto
                    radius: Look.linea
                    color: Look.luna
                }

                Anillo {
                    visible: word.keyed
                    corner: 0
                }

                HoverHandler {
                    id: hover

                    cursorShape: Qt.PointingHandCursor
                }

                TapHandler {
                    onTapped: filtro.pick(word.index)
                }
            }
        }
    }

    function pick(index) {
        keyed = index;
        if (index !== current)
            picked(index);
    }

    onCurrentChanged: keyed = current
    Keys.onLeftPressed: keyed = Math.max(0, keyed - 1)
    Keys.onRightPressed: keyed = Math.min(repeater.count - 1, keyed + 1)
    Keys.onReturnPressed: pick(keyed)
    Keys.onEnterPressed: pick(keyed)
    Keys.onSpacePressed: pick(keyed)
}
