import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: captura

    property var keys: []
    property bool lit: false
    readonly property bool listening: activeFocus
    readonly property color ink: lit ? Look.noche : Look.escarcha

    signal captured(var keys)
    signal done

    function listen() {
        forceActiveFocus(Qt.TabFocusReason);
    }

    function named(event) {
        const special = {
            [Qt.Key_Space]: "Space",
            [Qt.Key_Tab]: "Tab",
            [Qt.Key_Return]: "Enter",
            [Qt.Key_Backspace]: "Backspace",
            [Qt.Key_Delete]: "Delete",
            [Qt.Key_Home]: "Home",
            [Qt.Key_End]: "End",
            [Qt.Key_Left]: "Left",
            [Qt.Key_Right]: "Right",
            [Qt.Key_Up]: "Up",
            [Qt.Key_Down]: "Down"
        };
        if (special[event.key])
            return special[event.key];
        if (event.key >= Qt.Key_F1 && event.key <= Qt.Key_F12)
            return "F" + (event.key - Qt.Key_F1 + 1);
        if (event.key > 0x20 && event.key < 0x7f)
            return String.fromCharCode(event.key);
        return "";
    }

    implicitWidth: line.implicitWidth + 2 * Look.e3
    implicitHeight: Look.altoControl
    Accessible.role: Accessible.Button
    Accessible.name: listening || keys.length === 0 ? App.tr("Press the combination…") : keys.join("+")
    Keys.onPressed: event => {
        event.accepted = true;
        if (event.key === Qt.Key_Escape)
            return done();
        const key = named(event);
        if (key === "")
            return;
        const held = [[Qt.MetaModifier, "Meta"], [Qt.ControlModifier, "Ctrl"], [Qt.AltModifier, "Alt"], [Qt.ShiftModifier, "Shift"]];
        const combination = held.filter(pair => event.modifiers & pair[0]).map(pair => pair[1]).concat([key]);
        captured(combination);
        done();
    }

    Canvas {
        id: edge

        readonly property color tone: captura.lit ? Look.noche : Look.brumaBaja

        anchors.fill: parent
        onToneChanged: requestPaint()
        onWidthChanged: requestPaint()
        onPaint: {
            const draw = getContext("2d");
            draw.reset();
            draw.strokeStyle = tone;
            draw.lineWidth = Look.linea;
            draw.setLineDash([3, 3]);
            draw.beginPath();
            draw.roundedRect(0.5, 0.5, width - 1, height - 1, Look.radioChico, Look.radioChico);
            draw.stroke();
        }
    }

    Row {
        id: line

        anchors.centerIn: parent
        spacing: Look.listaHueco

        Text {
            visible: captura.listening || captura.keys.length === 0
            anchors.verticalCenter: parent.verticalCenter
            text: App.tr("Press the combination…")
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: captura.ink
        }

        Repeater {
            model: captura.listening ? [] : captura.keys

            Row {
                required property string modelData
                required property int index

                anchors.verticalCenter: parent.verticalCenter
                spacing: Look.listaHueco

                Text {
                    visible: parent.index > 0
                    anchors.verticalCenter: parent.verticalCenter
                    text: "+"
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tCuerpo
                    color: captura.ink
                }

                Rectangle {
                    implicitWidth: kbd.implicitWidth + 2 * 7
                    implicitHeight: kbd.implicitHeight + 2 * 3
                    radius: Look.radioChico
                    color: captura.lit ? Look.noche : Look.nocheAlta
                    border.width: Look.linea
                    border.color: captura.lit ? Look.noche : Look.nocheLinea

                    Text {
                        id: kbd

                        anchors.centerIn: parent
                        text: parent.parent.modelData
                        font.family: Look.letraTexto
                        font.pixelSize: Look.tTecla
                        font.weight: Font.DemiBold
                        color: Look.escarcha
                    }
                }
            }
        }
    }
}
