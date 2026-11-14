import QtQuick
import io.github.wallpaper_rs.app

Text {
    property bool lit
    property bool small

    textFormat: Text.PlainText
    wrapMode: Text.WordWrap
    font.family: Look.letraTexto
    font.pixelSize: small ? Look.tPie : Look.tCuerpo
    color: lit ? Look.escarcha : Look.bruma
}
