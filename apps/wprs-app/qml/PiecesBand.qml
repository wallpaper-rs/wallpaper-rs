import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: board

    readonly property bool es: Qt.locale().name.startsWith("es")
    readonly property var files: Qt.application.arguments.filter(a => /\.(jpg|png)$/.test(a))
    property alias frame: frame
    property alias menu: menu
    property alias page: bandPage

    function say(en, es) {
        return board.es ? es : en;
    }

    function pic(name) {
        const found = files.find(f => f.indexOf("/" + name + "/") >= 0);
        return found ? "file://" + found : "";
    }

    function cover(name, en, es, extra) {
        return Object.assign({
                "picture": pic(name),
                "name": say(en, es)
            }, extra ?? {});
    }

    Image {
        anchors.fill: parent
        source: board.files.length > 0 ? "file://" + board.files[0] : ""
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }

    Column {
        id: side

        x: Look.margen
        y: Look.margenArriba
        z: 2
        spacing: Look.compacto ? Look.e5 : Look.e6

        Logo {
        }

        WordMenu {
            id: menu

            current: 0
        }
    }

    TitleFrame {
        id: frame

        anchors.fill: parent
        page: bandPage
        sideBottom: side.y + side.height
        none: false
    }

    PiecesBandPage {
        id: bandPage

        parent: frame.seat
        anchors.fill: parent
        focus: true
        sheet: board
    }

    Hints {
        z: 3
        x: frame.hintsAt.x - width
        y: frame.hintsAt.y - height
        model: [{
                "keys": ["left", "right"],
                "text": App.tr("Move around")
            }, {
                "keys": ["Esc"],
                "text": App.tr("Menu")
            }]
    }
}
