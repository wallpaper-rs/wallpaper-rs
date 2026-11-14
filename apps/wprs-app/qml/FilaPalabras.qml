import QtQuick
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

Row {
    id: palabras

    required property Item fila
    required property bool lit

    visible: fila.words.length > 0
    spacing: Look.e3

    Repeater {
        model: palabras.fila.words

        delegate: Row {
            id: word

            required property var modelData
            required property int index
            readonly property bool on: palabras.fila.lit && palabras.fila.words.length > 1 && index === palabras.fila.wordAt

            spacing: Look.e1

            IconImage {
                visible: word.modelData.icon !== undefined
                anchors.verticalCenter: parent.verticalCenter
                name: word.modelData.icon ?? ""
                sourceSize: Qt.size(Look.icono, Look.icono)
                color: palabras.lit ? Look.noche : Look.escarcha
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: word.modelData.text
                font.family: Look.letraTexto
                font.pixelSize: Look.tValor
                font.weight: Font.Bold
                font.underline: word.on
                color: palabras.lit ? Look.noche : Look.escarcha
            }

            TapHandler {
                onTapped: {
                    palabras.fila.forceActiveFocus(Qt.MouseFocusReason);
                    palabras.fila.wordAt = word.index;
                    palabras.fila.worded(word.index);
                }
            }
        }
    }
}
