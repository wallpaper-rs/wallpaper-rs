import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: start

    component Path: Column {
        id: path

        property string title
        property string text
        property bool main

        signal chosen

        spacing: Look.e2
        activeFocusOnTab: true
        Accessible.role: Accessible.Button
        Accessible.name: title
        Accessible.description: text
        Accessible.onPressAction: chosen()
        Keys.onReturnPressed: chosen()
        Keys.onEnterPressed: chosen()

        Text {
            text: path.title
            font.family: Look.letraTitulo
            font.pixelSize: path.main ? Look.tGrande : Look.tGrande - 2
            font.weight: path.main ? Font.Bold : Font.Normal
            font.letterSpacing: -0.02 * Look.tGrande
            font.underline: path.activeFocus || hover.hovered
            color: Look.escarcha
        }

        Text {
            width: path.main ? Look.tapaFoco : Look.tapaFoco * 0.9
            text: path.text
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }

        HoverHandler {
            id: hover

            cursorShape: Qt.PointingHandCursor
        }

        TapHandler {
            onTapped: path.chosen()
        }
    }

    required property var run
    required property var adder
    required property var page
    readonly property bool banded: (page.band?.width ?? 0) > 0
    readonly property real low: Math.max(explore.height, side.height) + Look.e4 + (shelf.visible ? shelf.implicitHeight : 0)
    readonly property real high: upperBlock.implicitHeight
    readonly property alias shelf: shelf
    readonly property var examples: run.shelf.slice(0, 12).map(pkg => ({
                "picture": run.previewed[run.spec(pkg)] ? App.backend.url(run.previewed[run.spec(pkg)]) : "",
                "name": pkg.title,
                "caption": pkg.kind === "scene" ? App.tr("Scene · Bundled examples") : App.tr("Bundled examples")
            }))
    readonly property var hints: [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": [App.trc("key", "Enter")],
            "text": App.tr("Set as wallpaper")
        }, {
            "keys": [],
            "text": App.tr("You can also drop your files here")
        }]

    function put() {
        if (shelf.currentIndex >= 0 && shelf.currentIndex < run.shelf.length)
            run.place(run.shelf[shelf.currentIndex], false);
    }

    Column {
        id: upperBlock

        x: start.banded ? start.page.upper.x + Math.max(0, start.page.upper.width - width) : 0
        y: start.banded ? start.page.upper.y : 0
        width: Math.min(Look.tapaFoco * 2, start.banded ? start.page.upper.width : start.width)
        spacing: Look.e2

        Text {
            width: parent.width
            horizontalAlignment: start.banded ? Text.AlignRight : Text.AlignLeft
            text: App.tr("You have no wallpapers yet")
            wrapMode: Text.WordWrap
            font.family: Look.letraTitulo
            font.pixelSize: Look.tPagina
            font.weight: Font.Bold
            font.letterSpacing: -0.03 * Look.tPagina
            color: Look.escarcha
        }

        Text {
            width: parent.width
            horizontalAlignment: start.banded ? Text.AlignRight : Text.AlignLeft
            text: App.tr("Choose where to start: the first wallpaper that enters your library becomes the wallpaper of your desktop.")
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }
    }

    Item {
        id: lower

        x: start.banded ? start.page.band.x : 0
        y: start.banded ? start.page.band.y + Math.max(0, start.page.band.height - height) : upperBlock.height + Look.e6
        width: start.banded ? start.page.band.width : start.width
        implicitHeight: shelf.y + (shelf.visible ? shelf.implicitHeight : 0)
        height: implicitHeight

        Path {
            id: explore

            main: true
            title: App.tr("Explore repositories")
            text: App.tr("Ready-made scenes to set, starting with the bundled examples.")
            onChosen: applicationWindow().showPage(4)
        }

        Row {
            id: side

            x: start.banded ? parent.width - width : 0
            y: start.banded ? Math.max(0, explore.height - height) : explore.height + Look.e5
            spacing: Look.e6

            Path {
                title: App.tr("Add a source")
                text: App.tr("Pictures that renew themselves, like a picture of the day.")
                onChosen: applicationWindow().showPage(3)
            }

            Path {
                title: App.tr("Add a folder")
                text: App.tr("The pictures and videos you already have on this computer.")
                onChosen: start.adder.open()
            }
        }

        Estante {
            id: shelf

            visible: start.examples.length > 0
            y: Math.max(explore.height, side.y + side.height) + Look.e4
            width: parent.width
            height: implicitHeight
            focus: true
            label: App.tr("Bundled examples")
            model: start.examples
            actions: Row {
                spacing: Look.e3

                QQC2.Button {
                    highlighted: true
                    icon.name: "image"
                    text: App.tr("Set as wallpaper")
                    onClicked: start.put()
                }

                PiecesTecla {
                    anchors.verticalCenter: parent.verticalCenter
                    text: App.trc("key", "Enter")
                }
            }
            onActivated: start.put()
        }
    }
}
