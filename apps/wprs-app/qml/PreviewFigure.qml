import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: figure

    property var item: null
    property int size: PreviewLook.tira
    property string caption
    property bool using: false
    property bool focused: false

    spacing: Look.e2
    Accessible.role: Accessible.Graphic
    Accessible.name: caption

    Item {
        width: figure.size
        height: Math.round(figure.size * 9 / 16)

        PreviewPicture {
            anchors.fill: parent
            corner: Look.radioChico
            item: figure.item
        }

        Anillo {
            visible: figure.focused
            corner: Look.radioChico
        }

        Uso {
            visible: figure.using
            x: Look.e2
            y: Look.e2
        }
    }

    Text {
        width: figure.size
        text: figure.caption
        elide: Text.ElideRight
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        font.weight: figure.focused ? Font.DemiBold : Font.Normal
        color: figure.focused ? Look.escarcha : Look.bruma
    }
}
