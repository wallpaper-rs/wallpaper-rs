import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    readonly property alias menu: menu
    readonly property alias question: question
    readonly property alias confirm: confirm

    function enter() {
        menu.open();
        question.open();
        confirm.open();
    }

    onVisibleChanged: visible || [menu, question, confirm].forEach(panel => panel.close())

    Row {
        spacing: Look.e8

        QQC2.Button {
            id: fit

            text: qsTr("Fill the screen")
            onClicked: menu.open()
        }

        QQC2.Button {
            id: ask

            text: qsTr("Set")
            onClicked: question.open()
        }

        QQC2.Button {
            id: remove

            text: qsTr("Remove 3 wallpapers")
            onClicked: confirm.open()
        }
    }

    ListaMenu {
        id: menu

        opener: fit
        current: 1
        options: [
            {
                "text": qsTr("Fit"),
                "icon": "frame-corners"
            },
            {
                "text": qsTr("Fill the screen"),
                "icon": "arrows-out",
                "note": qsTr("current")
            },
            {
                "text": qsTr("Stretch"),
                "icon": "arrows-horizontal"
            },
            {
                "separator": true
            },
            {
                "text": qsTr("Tile"),
                "icon": "squares-four",
                "note": qsTr("images only"),
                "off": true
            }
        ]
    }

    PanelChico {
        id: question

        opener: ask
        title: qsTr("“Pulso” asks for permission")
        text: qsTr("It asks once; you can change it later in its options.")
        quiet: qsTr("Not now")
        principal: qsTr("Allow")

        Pide {
            width: question.width - 2 * Look.e4
            icon: "speaker-high"
            text: qsTr("Listen to the computer’s sound")
            reason: qsTr("To move with the music that plays.")
        }

        Pide {
            width: question.width - 2 * Look.e4
            icon: "globe"
            text: qsTr("Connect to casa.example")
            reason: qsTr("To bring the weather of your city.")
        }
    }

    PanelChico {
        id: confirm

        opener: remove
        title: qsTr("Remove 3 wallpapers?")
        text: qsTr("They leave your library. The files in your folders are not deleted.")
        quiet: qsTr("Cancel")
        principal: qsTr("Remove")
        principalIcon: "trash"
        borra: true

        Progreso {
            width: confirm.width - 2 * Look.e4
            text: qsTr("Removing…")
            count: qsTr("%1 of %2").arg(2).arg(3)
            value: 2 / 3
        }
    }
}
