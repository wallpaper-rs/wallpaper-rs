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
        spacing: Look.panelChico

        QQC2.Button {
            id: fit

            text: App.tr("Fill the screen")
            onClicked: menu.open()
        }

        QQC2.Button {
            id: ask

            text: App.tr("Set")
            onClicked: question.open()
        }

        QQC2.Button {
            id: remove

            text: App.tr("Remove 3 wallpapers")
            onClicked: confirm.open()
        }
    }

    ListaMenu {
        id: menu

        opener: fit
        current: 1
        options: [
            {
                "text": App.tr("Fit"),
                "icon": "frame-corners"
            },
            {
                "text": App.tr("Fill the screen"),
                "icon": "arrows-out",
                "note": App.tr("current")
            },
            {
                "text": App.tr("Stretch"),
                "icon": "arrows-horizontal"
            },
            {
                "separator": true
            },
            {
                "text": App.tr("Tile"),
                "icon": "squares-four",
                "note": App.tr("images only"),
                "off": true
            }
        ]
    }

    PanelChico {
        id: question

        opener: ask
        title: App.tr("“Pulso” asks for permission")
        text: App.tr("It asks once; you can change it later in its options.")
        quiet: App.tr("Not now")
        principal: App.tr("Allow")

        Pide {
            width: question.width - 2 * Look.e4
            icon: "speaker-high"
            text: App.tr("Listen to the computer’s sound")
            reason: App.tr("To move with the music that plays.")
        }

        Pide {
            width: question.width - 2 * Look.e4
            icon: "globe"
            text: App.tr("Connect to casa.example")
            reason: App.tr("To bring the weather of your city.")
        }
    }

    PanelChico {
        id: confirm

        opener: remove
        title: App.tr("Remove 3 wallpapers?")
        text: App.tr("They leave your library. The files in your folders are not deleted.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true

        Progreso {
            width: confirm.width - 2 * Look.e4
            text: App.tr("Removing…")
            count: App.tr("%1 of %2", 2, 3)
            value: 2 / 3
        }
    }
}
