import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: recent

    required property var page
    readonly property alias row: row
    readonly property bool any: page.projects.length > 0

    implicitHeight: any ? seguir.height : invita.height

    Column {
        id: seguir

        visible: recent.any
        anchors.right: parent.right
        width: Math.min(parent.width, row.contentWidth)
        spacing: Look.e3

        Text {
            x: row.pad
            text: App.tr("Continue with yours")
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFila
            font.weight: Font.Bold
            font.letterSpacing: Look.tFila * Look.apretado
            color: Look.escarcha
        }

        CreateRow {
            id: row

            width: parent.width
            height: implicitHeight
            grows: true
            label: App.tr("Continue with yours")
            model: recent.page.facts.recentModel
            active: recent.page.focused === row
            onTaken: recent.page.focused = row
            onActivated: recent.page.act(row)
        }
    }

    Column {
        id: invita

        visible: !recent.any
        anchors.right: parent.right
        width: Math.min(parent.width, Look.avisoAncho + Look.e7 * 2)
        spacing: Look.e3

        Text {
            width: parent.width
            horizontalAlignment: Text.AlignRight
            wrapMode: Text.WordWrap
            text: App.tr("Your first wallpaper starts here")
            font.family: Look.letraTitulo
            font.pixelSize: Look.tGrande
            font.weight: Font.Bold
            font.letterSpacing: Look.tGrande * Look.apretado
            color: Look.escarcha
        }

        Text {
            width: parent.width
            horizontalAlignment: Text.AlignRight
            wrapMode: Text.WordWrap
            text: App.tr("The easiest way is to edit a copy of an example scene and change it to your liking. To start from scratch, choose “Start a new one”.")
            font.family: Look.letraTexto
            font.pixelSize: Look.tExplica
            color: Look.bruma
        }
    }
}
