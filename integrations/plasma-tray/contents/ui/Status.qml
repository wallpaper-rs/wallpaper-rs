import QtQuick
import QtQuick.Layouts
import "../look"

RowLayout {
    property bool paused: false

    spacing: Look.e1 + 2

    Glyph {
        name: parent.paused ? "pause-fill" : "play-fill"
        width: Look.iconoNota
        height: width
        tone: parent.paused ? Look.ambar : Look.bruma
    }

    Words {
        text: parent.paused ? i18n("Paused") : i18n("Playing")
        soft: true
        font.pixelSize: Look.tPie
    }
}
