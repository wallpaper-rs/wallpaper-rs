import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: start

    required property var run
    required property var tools
    required property var adder

    spacing: Look.e5

    Vacio {
        title: App.tr("You have no wallpapers yet")
        text: App.tr("Choose where to start: the first wallpaper that enters your library becomes the wallpaper of your desktop.")
    }

    Row {
        spacing: Look.e3

        QQC2.Button {
            highlighted: true
            text: App.tr("Explore repositories")
            icon.name: "magnifying-glass"
            onClicked: applicationWindow().showPage(4)
        }

        QQC2.Button {
            text: App.tr("Add a source")
            icon.name: "plus"
            onClicked: applicationWindow().showPage(3)
        }

        QQC2.Button {
            text: App.tr("Add a folder")
            icon.name: "folder"
            onClicked: start.adder.open()
        }
    }

    Row {
        spacing: Look.e4
        visible: start.run.shelf.length > 0

        Repeater {
            model: start.run.shelf.slice(0, 6)

            Tapa {
                required property var modelData
                required property int index

                name: modelData.title
                note: index === 0 ? App.tr("Set as wallpaper") : ""
                picture: start.run.previewed[start.run.spec(modelData)] ? App.backend.url(start.run.previewed[start.run.spec(modelData)]) : ""
                lit: index === 0
                onTapped: start.run.place(modelData, false)
            }
        }
    }

    Text {
        text: App.tr("You can also drop your files here.")
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }
}
