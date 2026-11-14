import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

RowLayout {
    id: valor

    required property Item fila
    readonly property bool lit: fila.lit && !fila.off
    readonly property color quiet: lit ? Look.noche : fila.off ? (fila.lit ? Look.bruma : Look.brumaBaja) : Look.brumaBaja
    readonly property color dim: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.25)

    function begin() {
        if (fila.kind === "campo")
            campo.forceActiveFocus(Qt.TabFocusReason);
        else
            captura.listen();
    }

    function back(item) {
        item.focus = false;
        fila.forceActiveFocus(Qt.TabFocusReason);
    }

    Layout.alignment: Qt.AlignVCenter
    spacing: Look.valorHueco

    component Caret: IconImage {
        required property string glyph
        property int by: 0

        name: glyph
        sourceSize: Qt.size(Look.icono, Look.icono)
        color: valor.quiet

        TapHandler {
            enabled: parent.by !== 0
            onTapped: {
                valor.fila.forceActiveFocus(Qt.MouseFocusReason);
                valor.fila.step(parent.by);
            }
        }
    }

    component Words: Text {
        font.family: Look.letraTexto
        font.pixelSize: Look.tValor
        font.weight: valor.lit ? Font.Bold : Font.Medium
        color: valor.fila.soft
    }

    Caret {
        visible: valor.fila.kind === "elige"
        glyph: "caret-left"
        by: -1
    }

    QQC2.Slider {
        visible: valor.fila.kind === "riel"
        implicitWidth: Look.rielAncho
        focusPolicy: Qt.NoFocus
        enabled: !valor.fila.off
        from: valor.fila.from
        to: valor.fila.to
        stepSize: valor.fila.stepSize
        value: valor.fila.at
        palette.dark: valor.lit ? Look.noche : Look.escarcha
        palette.midlight: valor.lit ? valor.dim : Look.nocheLinea
        palette.window: valor.lit ? Look.noche : Look.escarcha
        palette.light: valor.lit ? Look.noche : Look.escarcha
        palette.mid: valor.lit ? Look.noche : Look.escarcha
        Accessible.ignored: true
        onMoved: {
            valor.fila.at = value;
            valor.fila.moved(value);
        }
    }

    Words {
        visible: valor.fila.kind === "riel" || valor.fila.kind === "despliega" ? false : text !== ""
        text: valor.fila.kind === "elige" ? valor.fila.options[valor.fila.current] ?? "" : valor.fila.value
    }

    FilaPalabras {
        fila: valor.fila
        lit: valor.lit
    }

    Words {
        visible: valor.fila.number !== ""
        text: valor.fila.number
        horizontalAlignment: Text.AlignRight
        Layout.minimumWidth: 3.2 * Look.tValor * 0.55
        color: valor.lit ? Look.noche : valor.fila.off ? valor.fila.soft : Look.escarcha
    }

    Rectangle {
        visible: valor.fila.kind === "llave"
        implicitWidth: Look.llaveAncho
        implicitHeight: Look.llaveAlto
        radius: height / 2
        color: valor.lit ? (valor.fila.checked ? Look.noche : valor.dim) : valor.fila.checked ? Look.escarcha : Look.nocheLinea

        Rectangle {
            readonly property real hueco: (parent.height - height) / 2

            x: valor.fila.checked ? parent.width - width - hueco : hueco
            y: hueco
            width: Look.llavePunto
            height: Look.llavePunto
            radius: height / 2
            color: valor.lit ? (valor.fila.checked ? Look.escarcha : Look.noche) : valor.fila.checked ? Look.noche : Look.bruma

            Behavior on x {
                enabled: valor.visible
                NumberAnimation {
                    duration: Look.paso
                    easing.type: Easing.BezierSpline
                    easing.bezierCurve: Look.pasoCurva
                }
            }
        }
    }

    QQC2.TextField {
        id: campo

        visible: valor.fila.kind === "campo"
        implicitWidth: Math.min(Look.campoAncho, Math.round(valor.fila.width / 2))
        implicitHeight: Look.altoControl
        activeFocusOnTab: false
        enabled: !valor.fila.off
        text: valor.fila.fieldText
        placeholderText: valor.fila.placeholder
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: valor.lit ? Look.noche : Look.escarcha
        placeholderTextColor: valor.lit ? Look.noche : Look.bruma
        palette.base: valor.lit ? "transparent" : Look.nocheAlta
        palette.text: valor.lit ? Look.noche : Look.escarcha
        palette.placeholderText: valor.lit ? Look.noche : Look.bruma
        palette.mid: valor.lit ? Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.35) : Look.nocheLinea
        palette.highlight: valor.lit ? Look.noche : Look.luna
        selectedTextColor: valor.lit ? Look.escarcha : Look.noche
        Accessible.name: valor.fila.text

        background: Rectangle {
            radius: Look.radioChico
            color: valor.lit ? "transparent" : Look.nocheAlta
            border.width: 1
            border.color: valor.lit ? Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.35) : Look.nocheLinea
        }
        Keys.onEscapePressed: valor.back(campo)
        Keys.onReturnPressed: valor.back(campo)
        Keys.onEnterPressed: valor.back(campo)
        Keys.onUpPressed: valor.fila.nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
        Keys.onDownPressed: valor.fila.nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
        onEditingFinished: {
            if (text !== valor.fila.fieldText) {
                valor.fila.fieldText = text;
                valor.fila.edited(text);
            }
        }
    }

    Captura {
        id: captura

        visible: valor.fila.kind === "captura"
        keys: valor.fila.keys
        lit: valor.lit
        onCaptured: combination => {
            valor.fila.keys = combination;
            valor.fila.captured(combination);
        }
        onDone: valor.back(captura)
    }

    Caret {
        visible: valor.fila.kind === "elige" || valor.fila.kind === "pliega" || valor.fila.kind === "despliega"
        glyph: valor.fila.kind === "despliega" && valor.fila.open ? "caret-down" : "caret-right"
        by: valor.fila.kind === "elige" ? 1 : 0
    }
}
