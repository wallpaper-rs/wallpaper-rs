import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: card

    required property var page
    readonly property var entry: page.entry
    readonly property bool hurt: !!entry && (entry.missing === true || String(entry.broken ?? "") !== "")
    readonly property string screen: App.outputs.length > 1 ? page.target : ""
    readonly property string main: hurt ? App.tr("Show in folder") : screen !== "" ? App.tr("Set as wallpaper on %1", App.screen(screen)) : App.tr("Set as wallpaper")

    function said(entry) {
        const gone = entry.missing === true;
        switch (entry.kind) {
        case "image":
            return gone ? App.tr("Could not open this picture: the file is missing") : App.tr("Could not open this picture: the file is damaged");
        case "video":
            return gone ? App.tr("Could not open this video: the file is missing") : App.tr("Could not open this video: the file is damaged");
        case "scene":
            return gone ? App.tr("Could not open this scene: the file is missing") : App.tr("Could not open this scene: the file is damaged");
        }
        return gone ? App.tr("Could not open this wallpaper: the file is missing") : App.tr("Could not open this wallpaper: the file is damaged");
    }

    spacing: Look.e3

    Nota {
        visible: card.hurt
        tone: "coral"
        text: card.entry ? card.said(card.entry) : ""
    }

    Row {
        spacing: Look.e3

        QQC2.Button {
            highlighted: true
            icon.name: card.hurt ? "folder-open" : "image"
            text: card.main
            onClicked: card.page.activate()
        }

        PiecesTecla {
            anchors.verticalCenter: parent.verticalCenter
            text: App.trc("key", "Enter")
        }

        QQC2.Button {
            flat: true
            anchors.verticalCenter: parent.verticalCenter
            text: App.tr("Options")
            icon.name: "caret-right"
            LayoutMirroring.enabled: true
            onClicked: card.page.open(card.entry.uid)
        }

        QQC2.Button {
            id: remove

            visible: card.hurt
            flat: true
            anchors.verticalCenter: parent.verticalCenter
            text: App.tr("Remove from the library…")
            onClicked: confirm.open()
        }
    }

    Detalles {
        visible: card.hurt && String(card.entry.broken ?? "") !== ""
        details: String(card.entry ? card.entry.broken ?? "" : "")
    }

    PanelChico {
        id: confirm

        opener: remove
        title: App.tr("Remove “%1” from the library?", card.entry ? card.entry.title ?? "" : "")
        text: App.tr("The file stays where it is.")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("Remove", [[card.entry.uid], false], "Library1")
    }
}
