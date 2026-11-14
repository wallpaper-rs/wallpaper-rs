import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

CreateFicha {
    id: ficha

    required property var page
    readonly property var entry: page.current
    readonly property bool starting: entry !== null && !page.onRecent && !page.atExamples
    readonly property alias principal: principal
    readonly property alias titleField: titleField
    readonly property string ask: starting && entry.id === "image" ? App.tr("Choose the picture…") : starting && entry.id === "video" ? App.tr("Choose the video…") : starting ? App.tr("Create") : page.missing ? App.tr("Find it…") : page.atExamples ? App.tr("Edit a copy…") : App.tr("Keep editing")

    title: entry ? entry.name : ""
    fields: starting && page.changing
    tone: page.missing ? "coral" : ""
    icon: page.missing ? "warning-circle" : ""
    caption: !entry ? "" : starting ? App.tr("It will be created as “%1” in %2", page.named, page.where) : page.missing ? App.tr("Its folder “%1” is gone. Did you move it or rename it?", entry.folder.split("/").slice(-1).join("")) : page.facts.pie

    function rename(on) {
        page.changing = on;
        if (on)
            titleField.forceActiveFocus(Qt.TabFocusReason);
        else if (page.focused)
            page.focused.forceActiveFocus(Qt.TabFocusReason);
    }

    middle: [
        Row {
            spacing: Look.e2

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Title")
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                color: Look.bruma
            }

            QQC2.TextField {
                id: titleField

                width: Look.avisoAncho * 0.45
                text: ficha.page.named
                Accessible.name: App.tr("Title")
                onTextEdited: ficha.page.named = text
                onAccepted: ficha.page.act(ficha.page.focused)
                Keys.onEscapePressed: ficha.rename(false)
            }
        },
        Row {
            spacing: Look.e2

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Folder")
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                color: Look.bruma
            }

            QQC2.TextField {
                width: Look.avisoAncho * 0.7
                text: paths.homed(ficha.page.placed !== "" ? ficha.page.placed : helper.folderFor(ficha.page.named))
                Accessible.name: App.tr("Folder")
                onTextEdited: ficha.page.placed = paths.expanded(text)
                onAccepted: ficha.page.act(ficha.page.focused)
                Keys.onEscapePressed: ficha.rename(false)
            }
        }
    ]

    QQC2.Button {
        visible: ficha.starting && !ficha.page.changing
        flat: true
        text: App.tr("Change…")
        onClicked: ficha.rename(true)
    }

    QQC2.Button {
        id: principal

        visible: ficha.entry !== null
        highlighted: true
        text: ficha.ask
        icon.name: ficha.starting ? (ficha.entry.id === "image" || ficha.entry.id === "video" ? "folder-open" : "plus") : ficha.page.missing ? "magnifying-glass" : ficha.page.atExamples ? "copy" : "pencil-simple"
        onClicked: ficha.page.act(ficha.page.focused)
    }

    PiecesTecla {
        text: App.trc("key", "Enter")
    }

    QQC2.Button {
        visible: ficha.page.missing
        flat: true
        text: App.tr("Remove from the list")
        onClicked: ficha.page.forget()
    }

    SceneEditor {
        id: helper
    }

    Recents {
        id: paths
    }
}
