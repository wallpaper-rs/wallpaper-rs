import QtQuick
import "../look"

Text {
    property bool title: false
    property bool soft: false

    color: soft ? Look.bruma : Look.escarcha
    font.family: title ? Look.letraTitulo : Look.letraTexto
    font.pixelSize: Look.tCuerpo
    font.weight: title ? Font.Bold : Font.Normal
    textFormat: Text.PlainText
    elide: wrapMode === Text.NoWrap ? Text.ElideRight : Text.ElideNone
}
